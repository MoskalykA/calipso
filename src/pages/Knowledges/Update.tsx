import { useEffect, useState, useRef } from "react"
import { invoke } from "@tauri-apps/api/tauri"
import { Link, useParams } from "react-router-dom"
 
function Update() {
    const { id } = useParams()
    const [name, setName] = useState("")
    const [image, setImage] = useState("")
    const [link, setLink] = useState("")
    const refName = useRef<HTMLInputElement>(null)
    const refImage = useRef<HTMLInputElement>(null)
    const refLink = useRef<HTMLInputElement>(null)
    useEffect(() => {
        invoke("request_knowledge_by_id", {
            id: Number(id)
        }).then((data: any) => {
            refName.current!.value = data.name
            refImage.current!.value = data.image
            refLink.current!.value = data.link
    
            setName(data.name)
            setImage(data.image)
            setLink(data.link)
        })
    }, [])

    return (
        <div className="flex justify-center items-center h-screen">
            <div className="bg-zinc-800/50 w-11/12 rounded-md cool-border flex flex-col">
                <div className="m-4 space-y-4">
                    <div>
                        <h1 className="font-mono">Name</h1>
                        <input ref={refName} onChange={(e) => {
                            setName(e.target.value)
                        }} className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0" type="text"/>
                    </div>

                    <div>
                        <h1 className="font-mono">Image</h1>
                        <input ref={refImage} onChange={(e) => {
                            setImage(e.target.value)
                        }} className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0" type="url"/>
                    </div>

                    <div>
                        <h1 className="font-mono">Link</h1>
                        <input ref={refLink} onChange={(e) => {
                            setLink(e.target.value)
                        }} className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0" type="url"/>
                    </div>
                </div>

                <div className="flex justify-center items-center">
                    <Link to={"/knowledges/index"} onClick={() => {
                        invoke("update_knowledge", {
                            id: Number(id),
                            name,
                            image,
                            link
                        })
                    }} className="cool-button font-mono mb-4 px-4">
                        <h1>Update</h1> 
                    </Link>
                </div>
            </div>
        </div>
    )
}

export default Update