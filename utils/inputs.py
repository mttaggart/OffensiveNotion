


def ask_for_input(message: str, default):
    """
    Args:
        message: the prompt message.
        default: the value if no answer is given to the prompt. No defined type.
    Returns:
        Value of the input or the default
    """
    result = input(message + " > ")
    if result == "":
        result = default
    return result


def yes_or_no(message, default) -> bool:
    """
    Simple method to ask yes or no.
    Args:
        message: prompt message.
    Returns: bool
    """
    while True:
        res = input(message)
        if res == "":
            res = default
        while res.lower() not in ("yes", "no"):
            print("[*] Please enter 'yes' or 'no' > ")
            res = input(message)
        if res == "no":
            return False
        else:
            return True
