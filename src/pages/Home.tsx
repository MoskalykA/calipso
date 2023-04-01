import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { FcIdea, FcFolder } from "react-icons/fc";

function Home() {
  const [knowledgeCount, setKnowledgeCount] = useState<number>(0);
  const [ideaCount, setIdeaCount] = useState<number>(0);

  useEffect(() => {
    invoke<number>("request_knowledge_count").then((data: number) => {
      setKnowledgeCount(data);
    });

    invoke<number>("request_idea_count").then((data: number) => {
      setIdeaCount(data);
    });
  }, []);

  return (
    <div className="flex">
      <div className="flex justify-between items-center p-4 m-4 w-full bg-zinc-800/50 rounded">
        <FcIdea className="w-5 h-5 text-white" />

        <h1 className="font-quicksand">
          {ideaCount === 0 ? `No idea` : `${ideaCount} ideas`}
        </h1>
      </div>

      <div className="flex justify-between items-center p-4 m-4 w-full bg-zinc-800/50 rounded">
        <FcFolder className="w-5 h-5 text-white" />

        <h1 className="font-quicksand">
          {knowledgeCount === 0
            ? `No knowledge`
            : `${knowledgeCount} knowledges`}
        </h1>
      </div>
    </div>
  );
}

export default Home;
