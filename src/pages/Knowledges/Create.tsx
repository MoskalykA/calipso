import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Link } from "react-router-dom";

function Create() {
  const [name, setName] = useState<string>("");
  const [image, setImage] = useState<string>("");
  const [link, setLink] = useState<string>("");

  return (
    <div className="flex justify-center items-center h-screen">
      <div className="bg-zinc-800/50 w-11/12 rounded-md flex flex-col">
        <div className="m-4 space-y-4">
          <div>
            <h1 className="font-quicksand">Name</h1>

            <input
              onChange={(e: any) => {
                setName(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 rounded font-quicksand text-white focus:outline-0"
              type="text"
            />
          </div>

          <div>
            <h1 className="font-quicksand">Image</h1>

            <input
              onChange={(e: any) => {
                setImage(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 rounded font-quicksand text-white focus:outline-0"
              type="url"
            />
          </div>

          <div>
            <h1 className="font-quicksand">Link</h1>

            <input
              onChange={(e: any) => {
                setLink(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 rounded font-quicksand text-white focus:outline-0"
              type="url"
            />
          </div>
        </div>

        <div className="flex justify-center items-center mt-8">
          <Link
            to={"/knowledges/index"}
            onClick={() => {
              invoke("add_knowledge", {
                name,
                image,
                link,
              });
            }}
            className="cool-button font-quicksand mb-4 px-4"
          >
            <h1>Create</h1>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default Create;
