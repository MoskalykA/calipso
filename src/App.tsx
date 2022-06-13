import {Route, Routes, Link} from "react-router-dom"
import {FcHome, FcIdea, FcSettings, FcFolder} from "react-icons/fc"
import Home from "./pages/Home"
import Knowledges from "./pages/Knowledges"

function App() {
    return (
        <div className="flex h-screen overflow-hidden">
            <div className="flex flex-col justify-between bg-zinc-800/50 p-4">
                <div className="space-y-1.5">
                    <Link to="/" className="button-header">
                        <FcHome/>
                        <h1>Home</h1>
                    </Link>

                    <Link to="/ideas" className="button-header">
                        <FcIdea/>
                        <h1>Ideas</h1>
                    </Link>

                    <Link to="/knowledges" className="button-header">
                        <FcFolder/>
                        <h1>Knowledges</h1>
                    </Link>
                </div>

                <div>
                    <Link to="/settings" className="button-header">
                        <FcSettings/>
                        <h1>Settings</h1>
                    </Link>
                </div>
            </div>

            <div className="w-full h-full flex flex-col overflow-auto">
                <Routes>
                    <Route path="/" element={<Home/>}/>
                    <Route path="/ideas" element={<Home/>}/>
                    <Route path="/knowledges" element={<Knowledges/>}/>

                    <Route path="/settings" element={<Home/>}/>
                </Routes>
            </div>
        </div>
    )
}

export default App