import sys
import smtplib
from email.mime.text import MIMEText

USERNAME = "adrian.arroyocalle@gmail.com"

'''
Uso de send_email.py
python3 send_email.py TITULO EMAIL_RESPUESTA GMAIL_PASSWORD
Pasar mensaje por stdin
'''
def main():
    msg = sys.stdin.read()
    msg = MIMEText(msg.encode("utf-8"),_subtype="html",_charset="utf-8")
    msg["Subject"] = sys.argv[1]
    msg["From"] = USERNAME
    msg["Cc"] = sys.argv[2]
    msg["To"] = USERNAME

    PASSWORD = sys.argv[3]

    s = smtplib.SMTP("smtp.gmail.com",587)
    s.ehlo()
    s.starttls()
    s.login(USERNAME,PASSWORD)
    s.sendmail(USERNAME,USERNAME,msg.as_string())
    s.quit()



if __name__ == "__main__":
    main()