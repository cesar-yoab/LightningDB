import socket

HOST = "127.0.0.1"
PORT = 6379

def main():
    """Creates the console and connects to the database"""
    # Connect to the server
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect((HOST, PORT))

    # Print initial ascii art
    print("="*10)
    print("LightningDB")
    print("|\\---/|")
    print("| o_o |")
    print(" \\_^_/")
    print("="*10)
    print("> ", end="")

    # Read commands from the user and send them to the server
    while True:
        # Read a command from the user
        command = input()
        if command == "EXIT":
            s.close()
            return

        # Send the command to the server
        s.sendall(command.encode())

        # Receive and display the server's response
        response = s.recv(1024)
        print(response.decode(), end="")

        # Print the prompt again
        print()
        print("> ", end="")

if __name__ == '__main__':
    main()
