import { FcAddImage } from "react-icons/fc";
import { Link } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import Knowledge from "@/types/Knowledge";

type Knowledges = Knowledge[];

function Index() {
  const [data, setData] = useState<Knowledges>([]);
  useEffect(() => {
    invoke<Knowledges>("request_knowledges").then((data: Knowledges) => {
      setData(data);
    });
  }, []);

  return (
    <div>
      <div className="mx-4 mt-4 p-4 bg-zinc-800/50 rounded w-fit cool-transition hover:cursor-pointer hover:bg-zinc-800">
        <Link to={"/knowledges/create"}>
          <FcAddImage />
        </Link>
      </div>

      <div className="grid grid-cols-2 gap-4 p-4">
        {data.map((knowledges: Knowledge, index: number) => (
          <div
            key={index}
            className="bg-zinc-800/50 rounded-lg hover:cursor-pointer"
          >
            <img className="rounded-t-lg" src={knowledges.image} alt="icon" />

            <div className="p-4 space-y-4 flex flex-col">
              <h1 className="text-center font-quicksand font-semibold leading-none">
                {knowledges.name}
              </h1>

              <div className="flex justify-center items-center space-x-4">
                <a
                  target="_blank"
                  href={knowledges.link}
                  className="cool-button font-quicksand font-normal"
                >
                  Open
                </a>

                <Link
                  to={`/knowledges/update/${index}`}
                  className="cool-button font-quicksand font-normal"
                >
                  <h1>Update</h1>
                </Link>

                <button
                  onClick={() => {
                    invoke("delete_knowledge", {
                      index,
                    }).then((data: any) => {
                      setData(data);
                    });
                  }}
                  className="cool-button font-quicksand font-normal"
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
