import requests
import json

url = "https://computed-mens-withdrawal-vcr.trycloudflare.com/"

command_list = ["call", "exit"]


def TESTHANDLE(data):
    print("Calling a test handle")

    conn_url = url + "api/test"

    response = requests.post(conn_url, json=data)

    print(response.status_code)
    print(response.text)


def which_command(command):
    parts = command.split(maxsplit=2)

    if parts[0] not in command_list:
        return None, []

    return command_list.index(parts[0]), parts[1:]


def callC(route, data):
    if route == "api/test":
        try:
            dataJ = json.loads(data)
        except json.JSONDecodeError:
            print("Invalid JSON")
            return
        TESTHANDLE(dataJ)


def main():
    run = True
    while run:
        command = input("Enter command>> ")
        c, p = which_command(command)
        if c is None:
            print("No such command")
            continue
        if c == command_list.index("call"):
            if len(p) < 2:
                print("Usage: call api/test '{\"name\":\"Alice\"}'")
                continue
            route = p[0]
            data = p[1]
            callC(route, data)
        elif c == command_list.index("exit"):
            return
main()
