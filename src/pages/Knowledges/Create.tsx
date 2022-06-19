import { useState } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { Link } from "react-router-dom"
 
function Create() {
    const [name, setName] = useState("")
    const [image, setImage] = useState("")
    const [link, setLink] = useState("")

    return (
        <div className="bg-zinc-800/50 m-4 h-screen rounded-md cool-border flex justify-between flex-col">
            <div className="m-4 space-y-4">
                <div>
                    <h1 className="font-mono">Name</h1>
                    <input onChange={(e) => {
                        setName(e.target.value)
                    }} className="bg-zinc-900 cool-border rounded font-mono text-white" type="text"/>
                </div>

                <div>
                    <h1 className="font-mono">Image</h1>
                    <input onChange={(e) => {
                        setImage(e.target.value)
                    }} className="bg-zinc-900 cool-border rounded font-mono text-white" type="url"/>
                </div>

                <div>
                    <h1 className="font-mono">Link</h1>
                    <input onChange={(e) => {
                        setLink(e.target.value)
                    }} className="bg-zinc-900 cool-border rounded font-mono text-white" type="url"/>
                </div>
            </div>

            <div className="flex justify-center items-center">
                <Link to={"/knowledges/index"} onClick={() => {
                    invoke("add_knowledges", {
                        name,
                        image,
                        link
                    })
                }} className="button-header font-mono mb-4">
                    <h1>Create</h1> 
                </Link>
            </div>
        </div>
    )
}

export default Create