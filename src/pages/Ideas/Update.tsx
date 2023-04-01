import { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Link, useParams } from "react-router-dom";
import Idea from "@/types/Idea";

function Update() {
  const { index } = useParams();

  const [name, setName] = useState<string>("");
  const [description, setDescription] = useState<string>("");

  const refName = useRef<HTMLInputElement>(null);
  const refDescription = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    invoke<Idea>("request_idea_by_id", {
      index: Number(index),
    }).then((data: Idea) => {
      refName.current!.value = data.name;
      refDescription.current!.value = data.description;

      setName(data.name);
      setDescription(data.description);
    });
  }, []);

  return (
    <div className="flex justify-center items-center h-screen">
      <div className="bg-zinc-800/50 w-11/12 rounded-md rounded flex flex-col">
        <div className="m-4 space-y-4">
          <div>
            <h1 className="font-mono">Name</h1>

            <input
              ref={refName}
              onChange={(e: any) => {
                setName(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 rounded rounded font-mono text-white focus:outline-0"
              type="text"
            />
          </div>

          <div>
            <h1 className="font-mono">Description</h1>

            <textarea
              rows={8}
              ref={refDescription}
              onChange={(e: any) => {
                setDescription(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 rounded rounded font-mono text-white focus:outline-0"
            />
          </div>
        </div>

        <div className="flex justify-center items-center">
          <Link
            to={"/ideas/index"}
            onClick={() => {
              invoke("update_idea", {
                index: Number(index),
                name,
                description,
              });
            }}
            className="cool-button font-mono mb-4 px-4"
          >
            <h1>Update</h1>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default Update;
