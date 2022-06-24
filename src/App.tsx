import { listen } from "@tauri-apps/api/event"
import { useState } from "react"
import { FaGithub } from "react-icons/fa"
import { Route, Routes, Link } from "react-router-dom"
import { FcHome, FcIdea, FcSettings, FcFolder } from "react-icons/fc"

import Home from "./pages/Home"

import IdeasIndex from "./pages/Ideas/Index"
import IdeasCreate from "./pages/Ideas/Create"
import IdeasUpdate from "./pages/Ideas/Update"
import IdeasView from "./pages/Ideas/View"

import KnowledgesIndex from "./pages/Knowledges/Index"
import KnowledgesCreate from "./pages/Knowledges/Create"
import KnowledgesUpdate from "./pages/Knowledges/Update"

import SettingsIndex from "./pages/Settings/Index"

function App() {
    const [error, setError] = useState<string | null>(null)
    listen("sendError", (e: { payload: string }) => {
        const parse = JSON.parse(e.payload)
        setError(parse.error)
    })

    return (
        <>
            { error ? (
                <div className="flex flex-col justify-center items-center h-screen">
                    <div className="w-1/2 p-4 flex flex-col space-y-2 bg-zinc-900 border border-zinc-700 rounded">
                        <h1 className="text-white font-mono">{ error }</h1>

                        <a href="https://github.com/MoskalykA/calypso" target="_blank" className="cool-button p-2 bg-zinc-800 hover:bg-zinc-700">
                            <FaGithub className="w-5 h-5 text-white"/>
                        </a>
                    </div>
                </div>
            ) : (
                <div className="flex h-screen overflow-hidden">
                    <div className="flex flex-col justify-between bg-zinc-800/50 p-4 cool-border rounded-r">
                        <div className="space-y-1.5">
                            <Link to="/" className="cool-button">
                                <FcHome/>
                                <h1>Home</h1>
                            </Link>

                            <Link to="/ideas/index" className="cool-button">
                                <FcIdea/>
                                <h1>Ideas</h1>
                            </Link>

                            <Link to="/knowledges/index" className="cool-button">
                                <FcFolder/>
                                <h1>Knowledges</h1>
                            </Link>
                        </div>

                        <div>
                            <Link to="/settings/index" className="cool-button">
                                <FcSettings/>
                                <h1>Settings</h1>
                            </Link>
                        </div>
                    </div>

                    <div className="w-full h-full flex flex-col overflow-auto">
                        <Routes>
                            <Route path="/" element={<Home/>}/>
                            <Route path="/ideas" element={<Home/>}/>

                            <Route path="/ideas/index" element={<IdeasIndex/>}/>
                            <Route path="/ideas/create" element={<IdeasCreate/>}/>
                            <Route path="/ideas/update/:id" element={<IdeasUpdate/>}/>
                            <Route path="/ideas/view/:id" element={<IdeasView/>}/>

                            <Route path="/knowledges/index" element={<KnowledgesIndex/>}/>
                            <Route path="/knowledges/create" element={<KnowledgesCreate/>}/>
                            <Route path="/knowledges/update/:id" element={<KnowledgesUpdate/>}/>

                            <Route path="/settings/index" element={<SettingsIndex/>}/>
                        </Routes>
                    </div>
                </div>
            )}
        </>
    )
}

export default App