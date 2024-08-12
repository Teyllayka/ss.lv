/// <references types="houdini-svelte">

/** @type {import('houdini').ConfigFile} */
const config = {
    "schemaPath": "./schema.graphql",
    "watchSchema": {
        "url": "http://localhost:80"
    },
    "plugins": {
        "houdini-svelte": {}
    }
}

export default config
