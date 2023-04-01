import { FcAddImage } from "react-icons/fc";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import Idea from "@/types/Idea";

type Ideas = Idea[];

function Index() {
  const [data, setData] = useState<Ideas>([]);
  useEffect(() => {
    invoke<Ideas>("request_ideas").then((data: Ideas) => {
      setData(data);
    });
  }, []);

  return (
    <div>
      <div className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit cool-transition hover:cursor-pointer hover:bg-zinc-800 rounded">
        <Link to={"/ideas/create"}>
          <FcAddImage />
        </Link>
      </div>

      <div className="grid grid-cols-2 gap-4 p-4">
        {data.map((idea: Idea, index: number) => (
          <div
            key={index}
            className="bg-zinc-800/50 rounded-lg hover:cursor-pointer rounded flex flex-col justify-center items-center"
          >
            <div className="p-4 space-y-4">
              <h1 className="text-center font-mono font-semibold leading-none">
                {idea.name}
              </h1>

              <div className="flex justify-center items-center space-x-4">
                <Link
                  to={`/ideas/view/${index}`}
                  className="cool-button font-mono font-normal"
                >
                  <h1>Open</h1>
                </Link>

                <Link
                  to={`/ideas/update/${index}`}
                  className="cool-button font-mono font-normal"
                >
                  <h1>Update</h1>
                </Link>

                <button
                  onClick={() => {
                    invoke("delete_idea", {
                      index: index,
                    }).then((data: any) => {
                      setData(data);
                    });
                  }}
                  className="cool-button font-mono font-normal"
                >
                  <h1>Delete</h1>
                </button>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Index;
