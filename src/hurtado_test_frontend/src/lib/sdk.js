import { createActor, canisterId } from '../../../declarations/hurtado_test_backend';
import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from '@dfinity/agent';
import { HOST, identityProvider } from './config';
export async function upload_file(file) {
    let auth = await AuthClient.create();
    let authenticated = await auth.isAuthenticated();
    if(!authenticated) {
        throw new Error("You are not authenticated")
    }

   let backend_actor = get_actor(auth.getIdentity())

    if (file.size > 1000000) {
        let chunks = chunkArrayBuffer(await file.arrayBuffer(), 1)
        let fd = -1;
        for (let index = 0; index < chunks.length; index++) {
            const element = chunks[index];
            if(index == 0) {
                let v = await backend_actor.create_file({
                    name: file.name,
                    id: 0,
                    owner: "",
                    data: new Uint8Array(element),
                })

                if(v.Err) {
                    throw new Error("Could not create File")
                }

                fd = Number(v.Ok)
            }

            if(fd == -1) {
                throw new Error("File Descriptor is invalid")
            }

            let rst = await backend_actor.add_chunk(BigInt(fd), new Uint8Array(element));
            if(rst.Err) {
                throw new Error("Error Adding Chunks")
            }

            
        }

        return fd
        
    } else {
        let rxt = await backend_actor.create_file({
            name: file.name,
            id: 0,
            owner: "",
            data: new Uint8Array(await file.arrayBuffer()),
        })

        if(rxt.Err) {
            throw new Error("Could not create File")
        }

        return Number(rxt.Ok)
    }


}

export async function logout () {
    let auth = await AuthClient.create()
    auth.logout()
}

export async function authenticate() {
    let auth = await AuthClient.create()

    await auth.login({
        identityProvider: identityProvider,
        maxTimeToLive: BigInt(3_600_000_000_000)
    })

}

export async function is_authenticated() {
    let client = await AuthClient.create()

    return await client.isAuthenticated()
}

export async function get_files() {
    let client = await AuthClient.create()
    if(!(await client.isAuthenticated())) {
        throw new Error("Not Authorized")
    }

    let backend_actor = get_actor(client.getIdentity())
    return await backend_actor.get_files()

}

function get_actor(identity) {
    let agent = new HttpAgent({
        identity: identity,
        host: HOST,
    })
    return createActor(canisterId, {
        agent
    })

}

export function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}


function chunkArrayBuffer(arrayBuffer, chunkSize) {
    const chunks = [];
    const byteSize = 1024 * 1024 * chunkSize; // chunkSize in MB
    for (let i = 0; i < arrayBuffer.byteLength; i += byteSize) {
      const end = Math.min(i + byteSize, arrayBuffer.byteLength);
      chunks.push(arrayBuffer.slice(i, end));
    }
    return chunks;
  }
  