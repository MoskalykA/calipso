import { invoke } from "@tauri-apps/api/tauri"
import { useEffect, useState } from "react"
import { FcIdea, FcFolder } from "react-icons/fc"

function Home() {
    const [knowledgeCount, setKnowledgeCount] = useState<number>(0)
    const [ideaCount, setIdeaCount] = useState<number>(0)
    useEffect(() => {
        invoke("request_knowledge_count").then((data: any) => {
            setKnowledgeCount(data)
        })

        invoke("request_idea_count").then((data: any) => {
            setIdeaCount(data)
        })
    }, [])

    return (
        <div className="flex">
            <div className="flex justify-between items-center p-4 m-4 w-full bg-zinc-800/50 cool-border">
                <FcIdea className="w-5 h-5 text-white"/>
                <h1 className="font-mono">{ ideaCount === 0 ? `${ideaCount} idea` : `${ideaCount} ideas` }</h1>
            </div>

            <div className="flex justify-between items-center p-4 m-4 w-full bg-zinc-800/50 cool-border">
                <FcFolder className="w-5 h-5 text-white"/>
                <h1 className="font-mono">{ knowledgeCount === 0 ? `${knowledgeCount} knowledge` : `${knowledgeCount} knowledges` }</h1>
            </div>
        </div>
    )
}

export default Home