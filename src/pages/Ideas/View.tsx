import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useParams } from "react-router-dom";
import Idea from "@/types/Idea";

function View() {
  const { index } = useParams();

  const refName = useRef<HTMLInputElement>(null);
  const refDescription = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    invoke<Idea>("request_idea_by_id", {
      index: Number(index),
    }).then((data: Idea) => {
      refName.current!.value = data.name;
      refDescription.current!.value = data.description;
    });
  }, []);

  return (
    <div className="flex justify-center items-center h-screen">
      <div className="bg-zinc-800/50 w-11/12 rounded-md flex flex-col">
        <div className="m-4 space-y-4">
          <div className="space-y-1">
            <h1 className="font-quicksand">Name</h1>

            <input
              ref={refName}
              className="p-1 w-full bg-zinc-900 rounded font-quicksand text-white focus:outline-0"
              type="text"
            />
          </div>

          <div className="space-y-1">
            <h1 className="font-quicksand">Description</h1>

            <textarea
              rows={8}
              ref={refDescription}
              className="p-1 w-full bg-zinc-900 rounded font-quicksand text-white focus:outline-0"
            />
          </div>
        </div>
      </div>
    </div>
  );
}

export default View;
