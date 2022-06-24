import { FcAddImage } from "react-icons/fc"
import { Link } from "react-router-dom"
import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"
import KnowledgeType from "../../types/Knowledge"

function Index() {
    const [data, setData] = useState<KnowledgeType[]>([])
    useEffect(() => {
        invoke("request_knowledges").then((data: any) => {
            setData(data)
        })
    }, [])

    return (
        <div>
            <div className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit cool-transition hover:cursor-pointer hover:bg-zinc-800 cool-border">
                <Link to={"/knowledges/create"}>
                    <FcAddImage/>
                </Link>
            </div>

            <div className="grid grid-cols-2 gap-4 p-4">
                {data.map((knowledges: { id: number, name: string, image: string, link: string }) => (
                    <div key={knowledges.id} className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border">
                        <img className="rounded-t-lg" src={knowledges.image} alt="icon"/>

                        <div className="p-4 space-y-4 flex flex-col">
                            <h1 className="text-center font-mono font-semibold leading-none">{knowledges.name}</h1>

                            <div className="flex justify-center items-center space-x-4">
                                <a target="_blank" href={knowledges.link} className="cool-button font-mono font-normal">Open</a>
                                
                                <button onClick={() => {
                                    invoke("delete_knowledge", {
                                        id: knowledges.id
                                    }).then((data: any) => {
                                        setData(data)
                                    })
                                }} className="cool-button font-mono font-normal">
                                    <h1>Delete</h1>
                                </button>

                                <Link to={"/knowledges/update/" + knowledges.id} className="cool-button font-mono font-normal">
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