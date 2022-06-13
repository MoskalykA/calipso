import { useState } from "react"
import { FcAddImage } from "react-icons/fc"

function Knowledges() {
    const [isVisible, setIsVisible] = useState(false)

    return isVisible ? (
        <div className="bg-zinc-800/50 m-4 h-screen rounded-md cool-border">
            a
        </div>
    ) : (
        <div>
             <div onClick={() => {
                setIsVisible(true)
             }} className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit transition-1000 hover:cursor-pointer hover:p-6 cool-border">
                <FcAddImage/>
            </div>

            <div className="grid grid-cols-2 gap-4 p-4">
                <div className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border">
                    <img className="rounded-t-lg" src="https://res.cloudinary.com/daily-now/image/upload/f_auto,q_auto/v1/posts/c27de1fe340099312eee7ef24b53f714" alt=""/>

                    <div className="p-4 space-y-4">
                        <h1 className="text-center font-mono font-semibold leading-none">What is Pair Programming and How Can It Help You Grow as a Developer</h1>

                        <div className="flex justify-center items-center space-x-4">
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Delete</button>
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Update</button>
                        </div>
                    </div>
                </div>

                <div className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border">
                    <img className="rounded-t-lg" src="https://res.cloudinary.com/daily-now/image/upload/f_auto,q_auto/v1/posts/c27de1fe340099312eee7ef24b53f714" alt=""/>

                    <div className="p-4 space-y-4">
                        <h1 className="text-center font-mono font-semibold leading-none">What is Pair Programming and How Can It Help You Grow as a Developer</h1>

                        <div className="flex justify-center items-center space-x-4">
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Delete</button>
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Update</button>
                        </div>
                    </div>
                </div>

                <div className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border">
                    <img className="rounded-t-lg" src="https://res.cloudinary.com/daily-now/image/upload/f_auto,q_auto/v1/posts/c27de1fe340099312eee7ef24b53f714" alt=""/>

                    <div className="p-4 space-y-4">
                        <h1 className="text-center font-mono font-semibold leading-none">What is Pair Programming and How Can It Help You Grow as a Developer</h1>

                        <div className="flex justify-center items-center space-x-4">
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Delete</button>
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Update</button>
                        </div>
                    </div>
                </div>

                <div className="bg-zinc-800/50 rounded-lg hover:cursor-pointer cool-border">
                    <img className="rounded-t-lg" src="https://res.cloudinary.com/daily-now/image/upload/f_auto,q_auto/v1/posts/c27de1fe340099312eee7ef24b53f714" alt=""/>

                    <div className="p-4 space-y-4">
                        <h1 className="text-center font-mono font-semibold leading-none">What is Pair Programming and How Can It Help You Grow as a Developer</h1>

                        <div className="flex justify-center items-center space-x-4">
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Delete</button>
                            <button className="p-2 px-4 bg-zinc-900 rounded font-mono transition-1000 hover:p-3 hover:px-6 cool-border">Update</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Knowledges