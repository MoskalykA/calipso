import { FcAddImage } from "react-icons/fc"
import { Link } from "react-router-dom"
import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"

function Index() {
    const [data, setData] = useState([])
    useEffect(() => {
        setTimeout(() => {
            invoke("request_ideas_data")
        }, 100)
    }, [])

    listen("sendIdeas", (e: { payload: string }) => {
        const parse = JSON.parse(e.payload)
        setData(parse)
    })

    return (
        <div>
            <div className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit transition-1000 hover:cursor-pointer hover:bg-zinc-800 cool-border">
                <Link to={"/ideas/create"}>
                    <FcAddImage/>
                </Link>
            </div>

            <div className="grid grid-cols-2 gap-4 p-4">
                {data.map((ideas: { id: number, name: string }) => (
                    <div key={ideas.id} className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border flex flex-col justify-center items-center">
                        <div className="p-4 space-y-4">
                            <h1 className="text-center font-mono font-semibold leading-none">{ideas.name}</h1>

                            <div className="flex justify-center items-center space-x-4">
                                <Link to={"/ideas/view/" + ideas.id} className="button-header font-mono font-normal">
                                    <h1>Open</h1>
                                </Link>
                                
                                <button onClick={() => {
                                    invoke("delete_idea_data", {
                                        id: ideas.id
                                    })
                                }} className="button-header font-mono font-normal">
                                    <h1>Delete</h1>
                                </button>

                                <Link to={"/ideas/update/" + ideas.id} className="button-header font-mono font-normal">
                                    <h1>Update</h1>
                                </Link>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}

export default Index