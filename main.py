#!/usr/bin/env python3
import os
import argparse

import subprocess as sub
from shutil import copyfile
from shutil import move

import utils
from utils.colors import *
from utils.inputs import *
import getpass
import json

parser = argparse.ArgumentParser(description='OffensiveNotion Setup. Must be run as root. Generates the '
                                             'OffensiveNotion agent in a container.')
# TODO: args for config, etc
args = parser.parse_args()

# Globals
curr_dir = os.getcwd()
config_file = curr_dir + "/config.json"
bin_dir = curr_dir + "/bin"


# Are you root?
def is_root():
    """
    Checks if the user is running the script with root privs. Exits if this is not the case. Root privs are needed to
    set up the Docker container used for compiling the agent.
    """
    if os.geteuid() == 0:
        return
    else:
        print("[-] You need to run this script as root!")
        parser.print_help()
        exit()


# Is docker installed?
def check_docker():
    """
    Checks if Docker is installed, exits if it is not.
    """
    print("[*] Checking Docker...")
    try:
        p = sub.Popen(['docker --version'], shell=True, stdin=sub.PIPE, stdout=sub.PIPE, stderr=sub.PIPE)
        out, err = p.communicate()
        if p.returncode == 0:
            print("[*] Docker is installed!")
        elif p.returncode > 0:
            print(
                "[*] Docker is not installed. Make sure to install Docker first (on Kali/Ubuntu, run: sudo apt-get "
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
    print("[*] Checking config file...")
    config_file_exists = os.path.exists(config_file)
    if not config_file_exists:
        print("[*] No config file located")
        return False
    else:
        print("[+] Config file located!")
        return True


def take_in_vars():
    # Sleep
    sleep_interval = ask_for_input("[*] Enter the sleep interval for the agent in seconds [default is 30s]", 30)
    print("[+] Sleep interval: {}".format(sleep_interval))
    # API Key
    api_key = getpass.getpass("[*] Enter your Notion Developer Account API key > ")
    print("[+] Got your API key!")
    # Parent Page ID
    print("[*] Your notion page's parent ID is the long number at the end of the URL.\n[*] For example, if your page "
          "URL is '[https://]www[.]notion[.]so/LISTENER-11223344556677889900112233445566', then your parent page ID is "
          "11223344556677889900112233445566")
    parent_page_id = input("[*] Enter your listener's parent page ID > ")
    print("[+] Parent page ID: {}".format(parent_page_id))
    json_vars = {
        "sleep": sleep_interval,
        "api_key": api_key,
        "parent_page_id": parent_page_id
    }
    json_string = json.dumps(json_vars)
    return json_string


def read_config():
    with open("config.json", "r") as jsonfile:
        data = json.load(jsonfile)
        jsonfile.close()
    for k, v in data.items():
        if k == "api_key":
            print(r"    [*] {}: [REDACTED]".format(k))
        else:
            print(r"    [*] {}: {}".format(k, v))


def write_config(json_string):
    with open('config.json', 'w') as outfile:
        outfile.write(json_string)


def are_configs_good() -> bool:
    res = utils.inputs.yes_or_no("[?] Do these look good? [yes/no] [default is yes] > ", "yes")
    return res


# When the configs look good:

def copy_source_file():
    print(info + "Creating agent's config source code...")
    source_dir = curr_dir + "/src/"
    src = source_dir + "config.rs"
    dst = source_dir + "config.rs.bak"
    copyfile(src, dst)


# TODO: SED configs into config src
def sed_source_code():
    print(info + "Setting variables in agent source...")
    print(important + "This function is under construction!")

# TODO: SED Dockerfile for build options (release, debug, etc) from args
def sed_dockerfile():
    print(info + "Setting dockerfile variables...")
    print(important + "This function is under construction!")

# Start Docker container, Dockerfile handles compilation
def docker_build():
    try:
        print(info + "Creating temporary build environment container...")
        sub.call(['docker rm offensivenotion -f 1>/dev/null 2>/dev/null && docker build -t offensivenotion .'],
                 shell=True)
    except Exception as e:
        print(printError + str(e))
        exit(1)


def docker_run():
    try:
        print(info + "Starting build container...")
        sub.call(['docker run --name offensivenotion -dt offensivenotion 1>/dev/null'], shell=True)
    except Exception as e:
        print(printError + str(e))
        exit(1)


# Copy agent out to physical system
def docker_copy():
    print(info + "Copying payload binary to host...")
    try:
        sub.call(['docker cp offensivenotion:/opt/OffensiveNotion/target/ bin/ 1>/dev/null'], shell=True)
        exists = os.path.isdir(bin_dir + "/target")
        if exists:
            print(good + "Success! Agents are located in the bin/ directory on the host.")
            return True
    except Exception as e:
        print(printError + str(e))
        exit(1)


# Tear down docker container
def docker_kill():
    print(info + "Removing temporary container...")
    try:
        sub.call(['docker rm offensivenotion -f 1>/dev/null'], shell=True)
    except Exception as e:
        print(printError + str(e))
        exit(1)


def recover_config_source():
    print(info + "Recovering original source code...")
    source_dir = curr_dir + "/src/"
    old_conf = source_dir + "config.rs.bak"
    curr_conf = source_dir + "config.rs"
    exists = os.path.isfile(old_conf)
    if exists:
        try:
            os.remove(curr_conf)
            move(old_conf, curr_conf)
        except Exception as e:
            print(printError + str(e))


# TODO: C2 check: make request to page with configs and see if the C2 works

def main():
    is_root()
    check_docker()

    # Config file checks
    configs = does_config_exist()
    if not configs:
        print("[*] Lets set up a config file")
        json_vars = take_in_vars()
        write_config(json_vars)

    read_config()
    looks_good = are_configs_good()

    while not looks_good:
        json_vars = take_in_vars()
        write_config(json_vars)
        read_config()
        looks_good = are_configs_good()

    print("[+] Config looks good!")

    copy_source_file()
    sed_source_code()
    sed_dockerfile()

    try:
        docker_build()
        docker_run()
        docker_copy()
        docker_kill()
    except Exception as e:
        print(printError + str(e))

    recover_config_source()
    print(good + "Done! Happy hacking!")


if __name__ == "__main__":
    main()
