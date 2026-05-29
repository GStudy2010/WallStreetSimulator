import requests
import json

url = "https://graphical-apartment-males-graduates.trycloudflare.com/"
command_list = ["call", "exit", "createuser", "loginuser", "logoutuser"]
session_token = None

def CREATEUSER(data):
    print("Creating user")
    conn_url = url + "api/createuser"
    response = requests.post(conn_url, json=data)
    print(response.status_code)
    print(response.text)

def LOGINUSER(data):
    global session_token
    print("Logging in")
    conn_url = url + "api/loginuser"
    response = requests.post(conn_url, json=data)
    print(response.status_code)
    print(response.text)
    try:
        resp_json = response.json()
        if "message" in resp_json:
            session_token = resp_json["message"]
            print(f"Session token saved: {session_token}")
    except json.JSONDecodeError:
        pass

def LOGOUT():
    global session_token
    if session_token is None:
        print("Not logged in")
        return
    print("Logging out")
    conn_url = url + "api/logoutuser"
    headers = {"Authorization": f"Bearer {session_token}"}
    response = requests.post(conn_url, headers=headers)
    print(response.status_code)
    print(response.text)
    session_token = None

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

def creatuserask():
    name = input("Give a name for the user >> ")
    email = input("Give an email for the user >> ")
    password = input("Give a password for the user >> ")
    data = {
        "name": name,
        "email": email,
        "password": password,
    }
    CREATEUSER(data)

def loginuserask():
    email = input("Give your email >> ")
    password = input("Give your password >> ")
    data = {
        "email": email,
        "password": password,
    }
    LOGINUSER(data)

def callC(route, data):
    if route == "api/test":
        try:
            dataJ = json.loads(data)
        except json.JSONDecodeError:
            print("Invalid JSON")
            return
        TESTHANDLE(dataJ)
    if route == "api/createuser":
        try:
            dataJ = json.loads(data)
        except json.JSONDecodeError:
            print("Invalid JSON")
            return

def main():
    run = True
    while run:
        logged_in = f"[logged in]" if session_token else "[not logged in]"
        command = input(f"{logged_in} Enter command>> ")
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
        elif c == command_list.index("createuser"):
            creatuserask()
        elif c == command_list.index("loginuser"):
            loginuserask()
        elif c == command_list.index("logout"):
            LOGOUT()

main()
