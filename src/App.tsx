import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { FaGithub } from "react-icons/fa";
import { FcHome, FcIdea, FcFolder } from "react-icons/fc";
import Home from "@/pages/Home";
import IdeasIndex from "@/pages/Ideas/Index";
import IdeasCreate from "@/pages/Ideas/Create";
import IdeasUpdate from "@/pages/Ideas/Update";
import IdeasView from "@/pages/Ideas/View";
import KnowledgesIndex from "@/pages/Knowledges/Index";
import KnowledgesCreate from "@/pages/Knowledges/Create";
import KnowledgesUpdate from "@/pages/Knowledges/Update";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/ideas/index",
    element: <IdeasIndex />,
  },
  {
    path: "/ideas/create",
    element: <IdeasCreate />,
  },
  {
    path: "/ideas/update/:index",
    element: <IdeasUpdate />,
  },
  {
    path: "/ideas/view/:index",
    element: <IdeasView />,
  },
  {
    path: "/knowledges/index",
    element: <KnowledgesIndex />,
  },
  {
    path: "/knowledges/create",
    element: <KnowledgesCreate />,
  },
  {
    path: "/knowledges/update/:index",
    element: <KnowledgesUpdate />,
  },
]);

function App() {
  const [error, setError] = useState<string | null>(null);
  listen(
    "sendError",
    (e: {
      payload: {
        error: string;
      };
    }) => {
      setError(e.payload.error);
    }
  );

  return (
    <>
      {error ? (
        <div className="flex flex-col justify-center items-center h-screen">
          <div className="w-1/2 p-4 flex flex-col space-y-2 bg-zinc-900 border border-zinc-700 rounded">
            <h1 className="text-white font-quicksand">{error}</h1>

            <a
              href="https://github.com/MoskalykA/calypso"
              target="_blank"
              className="cool-button p-2 bg-zinc-800 hover:bg-zinc-700"
            >
              <FaGithub className="w-5 h-5 text-white" />
            </a>
          </div>
        </div>
      ) : (
        <div className="flex h-screen overflow-hidden">
          <div className="flex flex-col bg-zinc-800/50 p-4 rounded-r-lg space-y-1.5">
            <a href="/" className="font-quicksand font-semibold cool-button">
              <FcHome />

              <h1>Home</h1>
            </a>

            <a
              href="/ideas/index"
              className="font-quicksand font-semibold cool-button"
            >
              <FcIdea />

              <h1>Ideas</h1>
            </a>

            <a
              href="/knowledges/index"
              className="font-quicksand font-semibold cool-button"
            >
              <FcFolder />

              <h1>Knowledges</h1>
            </a>
          </div>

          <div className="w-full h-full flex flex-col overflow-auto">
            <RouterProvider router={router} />
          </div>
        </div>
      )}
    </>
  );
}

export default App;
