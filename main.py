#!/usr/bin/env python3

# Are you root?

# Is docker installed?

# Is there a config file?
    # If no config file, let's set one up

# Do these configs look good?
    # If not, return to config file set up

# When the configs look good:
    # Start Docker container
    # Compile agent
    # Copy agent back out to bin/ dir
    # Tear down docker container

# C2 check: make request to page with configs and see if the C2 works


def main():
    print("Main!")


if __name__ == "__main__":
    main()
