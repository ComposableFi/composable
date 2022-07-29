import os
import time
import argparse
import sys
import json
import requests


def process_local_subwasm(chain='dali', path='/home/ubuntu/'):
    stream1 = str("subwasm -j info "+path+chain+"_runtime.wasm | jq -r .blake2_256")
    print(stream1)
    stream1 = os.popen(stream1)
    output1 = stream1.read()
    print(output1)

    return(output1)


def process_remote_subwasm(http_con_str='http://localhost:9933'):
    stream = str("subwasm -j info "+http_con_str+" | jq -r .blake2_256")
    print(stream)
    stream = os.popen(stream)
    output = stream.read()
    print(output)
    return(output)



def SendMessageSlack(message = 'test_message', col = "#36ee33", icon = ":cactus:", title = f"Monitoring Alert :zap: @here", secret_url = "****"):
    try:
        #title = ()
        slack_data = {
        "username": "NotificationBot",
        "icon_emoji": icon,
        #"channel" : "#somerandomcahnnel",
        "attachments": [
            {
                "color": col,
                "fields": [
                    {
                        "title": title,
                        "value": message,
                        "short": "false",
                    }
                ]
            }
            ]
        }
        byte_length = str(sys.getsizeof(slack_data))

        headers = {'Content-Type': "application/json", 'Content-Length': byte_length}

        response = requests.post(secret_url, data=json.dumps(slack_data), headers=headers)

    except:
        raise Exception(response.status_code, response.text)



def main():

    parser = argparse.ArgumentParser(description='wasm checker')
    parser.add_argument('--chain',choices=['dali', 'composable', 'picasso'])
    args = parser.parse_args(sys.argv[1:])

    if(args.chain == 'dali'):
        print('dali')
        local_wasm = process_local_subwasm(chain='dali', path='/home/ubuntu/')
        remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
        #local_wasm = remote_wasm
        while True:
            time.sleep(15)
            remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
            #remote_wasm == local_wasm
            local_wasm = remote_wasm
            print(local_wasm)
            print(remote_wasm)
            if remote_wasm == local_wasm:
                SendMessageSlack(message='wasm updated!')
                break


    if(args.chain == 'composable'):
        print('dali')
        local_wasm = process_local_subwasm(chain='composable', path='/home/ubuntu/')
        remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
        #local_wasm = remote_wasm
        while True:
            time.sleep(15)
            remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
            print(local_wasm)
            print(remote_wasm)
            if remote_wasm == local_wasm:
                SendMessageSlack(message='wasm updated!')
                break


    if(args.chain == 'picasso'):
        print('dali')
        local_wasm = process_local_subwasm(chain='picasso', path='/home/ubuntu/')
        remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
        #local_wasm = remote_wasm
        while True:
            time.sleep(15)
            remote_wasm = process_remote_subwasm(http_con_str='http://localhost:9933')
            #remote_wasm == local_wasm
            local_wasm = remote_wasm
            print(local_wasm)
            print(remote_wasm)
            if remote_wasm == local_wasm:
                SendMessageSlack(message='wasm updated!')
                break
if __name__ =="__main__":
    main()
