/// <references types="houdini-svelte">
// import { PUBLIC_BACKEND_URI } from '$env/static/public';


/** @type {import('houdini').ConfigFile} */
const config = {
    "schemaPath": "./schema.graphql",
    "watchSchema": {
        "url": "http://localhost:80",
    },
    "plugins": {
        "houdini-svelte": {}
    },
    scalars: {
        NaiveDateTime: {
            type: "Date"
        }
    }
  
}

export default config
