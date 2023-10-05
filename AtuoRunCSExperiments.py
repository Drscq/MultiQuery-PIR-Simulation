import paramiko
import os
import subprocess
import time

# Email
import smtplib
from email.message import EmailMessage
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from email.mime.base import MIMEBase
from email import encoders

def send_email_with_attachment():
    # Email configurations
    SMTP_SERVER = 'smtp.gmail.com'  # Your SMTP server
    SMTP_PORT = 587  # Port for SMTP (commonly 587 for TLS, 465 for SSL, 25 for non-secure)
    SENDER_EMAIL = ''  # Your email address
    SENDER_PASSWORD = ''  # Your email password
    RECEIVER_EMAIL = ''  # Your colleague's email address

    # Create a multipart email
    msg = MIMEMultipart()
    msg['From'] = SENDER_EMAIL
    msg['To'] = RECEIVER_EMAIL
    msg['Subject'] = 'Log File from Latest Run'

    # Email body
    body = 'Dear colleague, attached is the log file from our latest run.'
    msg.attach(MIMEText(body, 'plain'))

    # Attach the file
    filename = 'output.txt'
    with open(filename, 'rb') as attachment:
        part = MIMEBase('application', 'octet-stream')
        part.set_payload(attachment.read())
        encoders.encode_base64(part)
        part.add_header('Content-Disposition', f'attachment; filename= {filename}')
        msg.attach(part)

    # Sending the email
    with smtplib.SMTP(SMTP_SERVER, SMTP_PORT) as server:
        server.starttls()  # Upgrade the connection to secure encrypted SSL/TLS
        server.login(SENDER_EMAIL, SENDER_PASSWORD)
        server.sendmail(SENDER_EMAIL, RECEIVER_EMAIL, msg.as_string())
# Path constants
SERVER_UTIL_PATH = "/home/changqi/PIR/Piano-PIR/util/util.go"
SERVER_CONFIG_PATH = "/home/changqi/PIR/Piano-PIR/config.txt"
CLIENT_UTIL_PATH = "/Users/sunny/Documents/GitHub/Piano-PIR/util/util.go"
CLIENT_CONFIG_PATH = "/Users/sunny/Documents/GitHub/Piano-PIR/config.txt"

DB_ENTRY_SIZES = {
    "4KB": 4 * 1024,
    "16KB": 16 * 1024,
    "64KB": 64 * 1024,
    "256KB": 256 * 1024
}

DB_ENTRY_MAPPING = {
    "4KB": list(range(14, 25, 2)),
    "16KB": list(range(12, 23, 2)),
    "64KB": list(range(10, 21, 2)),
    "256KB": list(range(8, 19, 2))
}

# SSH constants
SSH_HOST = "asap.cs.vt.edu"
SSH_USER = "YourUsername"
SSH_PASS = "Password"  # Consider using an SSH key instead for security.

ssh_client = paramiko.SSHClient()
ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
ssh_client.connect(SSH_HOST, username=SSH_USER, password=SSH_PASS)

def replace_on_remote(filepath, target, replacement):
    # Fetch the file from the remote server
    sftp = ssh_client.open_sftp()
    sftp.get(filepath, 'tempfile.txt')
    sftp.close()

    # Modify the file locally
    replace_in_file('tempfile.txt', target, replacement)

    # Send the modified file back to the server
    sftp = ssh_client.open_sftp()
    sftp.put('tempfile.txt', filepath)
    sftp.close()

    os.remove('tempfile.txt')

def replace_in_file(filepath, target, replacement):
    with open(filepath, 'r') as file:
        data = file.read()
        data = data.replace(target, replacement)
    with open(filepath, 'w') as file:
        file.write(data)

# def start_remote_server():
#     stdin, stdout, stderr = ssh_client.exec_command("cd /home/changqi/PIR/Piano-PIR/ && go run server/server.go -port 8112 &")
#     print(stdout.read().decode())
#     print(stderr.read().decode())
#     # You might need to handle this differently based on how the remote server responds.
#     time.sleep(5)  # Give it some time to fully start
#     return stdout.channel
def start_remote_server():
    transport = ssh_client.get_transport()
    channel = transport.open_session()
    channel.exec_command("cd /home/changqi/PIR/Piano-PIR/ && go run server/server.go -port 8112 &")
    
    time.sleep(5)  # Give it some time to fully start
    return channel

# def start_client():
#     # (This remains unchanged from before)
#     cmd = ["go", "run", "/Users/sunny/Documents/GitHub/Piano-PIR/client/client.go", "-ip", f"{SSH_HOST}:8112 -thread 8"]
#     process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
#     for line in iter(process.stdout.readline, b''):
#         print(line.decode(), end='')
#         if "quit signal" in line.decode():
#             break
#     process.terminate()
def start_client():
    cmd = ["go", "run", "/Users/sunny/Documents/GitHub/Piano-PIR/client/client.go", "-ip", f"{SSH_HOST}:8112", "-thread", "1"]
    process = subprocess.Popen(cmd)
    process.wait()  # Wait for the command to finish

def stop_remote_server():
    # Identify the PID of the process using port 8112
    cmd_find_process = "lsof -t -i :8112"
    stdin, stdout, stderr = ssh_client.exec_command(cmd_find_process)
    pid = stdout.read().decode().strip()
    
    if pid:  # If a PID was found
        # Kill the process with the identified PID
        cmd_kill_process = f"kill -9 {pid}"
        ssh_client.exec_command(cmd_kill_process)
        print(f"Killed process with PID {pid} on port 8112.")
    else:
        print("No process found on port 8112.")
    
    time.sleep(2)  # Allow some time for the process to terminate

# The rest of the main() function remains mostly unchanged.
def main():
    pre_value = 262144
    pre_power = 18
    for size_str, size_val in DB_ENTRY_SIZES.items():
        # target log the previous value before size_val and replacement log the cur
        replace_on_remote(SERVER_UTIL_PATH, "DBEntrySize = " + str(pre_value), "DBEntrySize = " + str(size_val))
        replace_in_file(CLIENT_UTIL_PATH, f"DBEntrySize = {pre_value}", f"DBEntrySize = {size_val}")
        pre_value = size_val
        for entry_power in DB_ENTRY_MAPPING[size_str]:
            print(f"Setting DBEntrySize to {size_str} ({size_val} bytes) and number of entries to 2^{entry_power}")
            # Server side operations
            replace_on_remote(SERVER_CONFIG_PATH, str(2 ** pre_power) , str(2 ** entry_power))
            server_channel = start_remote_server()

            # Client side operations
            replace_in_file(CLIENT_CONFIG_PATH, str(2 ** pre_power), str(2 ** entry_power))
            pre_power = entry_power
            start_client()

            # Once done, kill the server
            stop_remote_server()
    # Send email
    send_email_with_attachment()


if __name__ == "__main__":
    stop_remote_server()
    main()
    ssh_client.close()