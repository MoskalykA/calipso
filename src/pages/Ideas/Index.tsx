import { FcAddImage } from "react-icons/fc"
import { Link } from "react-router-dom"
import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"
import IdeaType from "../../types/Idea"

function Index() {
    const [data, setData] = useState<IdeaType[]>([])
    useEffect(() => {
        invoke("request_ideas").then((data: any) => {
            setData(data)
        })
    }, [])

    return (
        <div>
            <div className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit cool-transition hover:cursor-pointer hover:bg-zinc-800 cool-border">
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
                                <Link to={"/ideas/view/" + ideas.id} className="cool-button font-mono font-normal">
                                    <h1>Open</h1>
                                </Link>
                                
                                <button onClick={() => {
                                    invoke("delete_idea", {
                                        id: ideas.id
                                    }).then((data: any) => {
                                        setData(data)
                                    })
                                }} className="cool-button font-mono font-normal">
                                    <h1>Delete</h1>
                                </button>

                                <Link to={"/ideas/update/" + ideas.id} className="cool-button font-mono font-normal">
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