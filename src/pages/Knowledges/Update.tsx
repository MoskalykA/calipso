import { useEffect, useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Link, useParams } from "react-router-dom";
import Knowledge from "@/types/Knowledge";

function Update() {
  const { index } = useParams();

  const [name, setName] = useState<string>("");
  const [image, setImage] = useState<string>("");
  const [link, setLink] = useState<string>("");

  const refName = useRef<HTMLInputElement>(null);
  const refImage = useRef<HTMLInputElement>(null);
  const refLink = useRef<HTMLInputElement>(null);

  useEffect(() => {
    invoke<Knowledge>("request_knowledge_by_id", {
      index: Number(index),
    }).then((data: Knowledge) => {
      refName.current!.value = data.name;
      refImage.current!.value = data.image;
      refLink.current!.value = data.link;

      setName(data.name);
      setImage(data.image);
      setLink(data.link);
    });
  }, []);

  return (
    <div className="flex justify-center items-center h-screen">
      <div className="bg-zinc-800/50 w-11/12 rounded-md cool-border flex flex-col">
        <div className="m-4 space-y-4">
          <div>
            <h1 className="font-mono">Name</h1>

            <input
              ref={refName}
              onChange={(e: any) => {
                setName(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0"
              type="text"
            />
          </div>

          <div>
            <h1 className="font-mono">Image</h1>

            <input
              ref={refImage}
              onChange={(e: any) => {
                setImage(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0"
              type="url"
            />
          </div>

          <div>
            <h1 className="font-mono">Link</h1>

            <input
              ref={refLink}
              onChange={(e: any) => {
                setLink(e.target.value);
              }}
              className="p-1 w-full bg-zinc-900 cool-border rounded font-mono text-white focus:outline-0"
              type="url"
            />
          </div>
        </div>

        <div className="flex justify-center items-center">
          <Link
            to={"/knowledges/index"}
            onClick={() => {
              invoke("update_knowledge", {
                index: Number(index),
                name,
                image,
                link,
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
