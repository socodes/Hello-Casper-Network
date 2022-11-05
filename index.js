const fs = require('fs')
const{RuntimeArgs, CLValueBuilder, Contracts, CasperClient, Keys} = require('casper-js-sdk')

const client = new CasperClient("http://89.58.31.92:7777/rpc")
const contract = new Contracts.Contract(client)
const keys = Keys.Ed25519.loadKeyPairFromPrivateFile("./keys/secret_key.pem")

const wasm = new Uint8Array(fs.readFileSync("contract/target/wasm32-unknown-unknown/release/contract.wasm"))

async function install() {
    const args = RuntimeArgs.fromMap({
        "message":CLValueBuilder.string("Hello Casper Network!")
    })

    const deploy = contract.install(
        wasm,
        args,
        "20000000000",
        keys.publicKey,
        "casper-test",
        [keys]
    )

    try{
        return (await client.putDeploy(deploy))
    }
    catch (error) {
        return error
    }
}

//install().then(deployHash => console.log(deployHash)).catch(error => console.log(error))

async function update_msg() {
    contract.setContractHash("hash-c8a852cea02810f47acfee62fae321b9947ae71873958d7037ac802efe2f370d")
    const args = RuntimeArgs.fromMap({
        "message":CLValueBuilder.string("Hello again!")
    })

    const deploy = contract.callEntrypoint(
        "update_msg" ,
        args,
        keys.publicKey,
        "casper-test",
        "1000000000",
        [keys]
    )
    try{
        return (await client.putDeploy(deploy))
    }
    catch (error) {
        return error
    }
}
//update_msg().then(deployHash => console.log(deployHash)).catch(error => console.log(error))
function queryMessage() {
    contract.setContractHash("hash-c8a852cea02810f47acfee62fae321b9947ae71873958d7037ac802efe2f370d")
    return contract.queryContractData(["message"])
}
queryMessage().then(result => console.log(result)).catch(error => console.log(error))
