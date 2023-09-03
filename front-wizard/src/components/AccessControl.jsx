import { Accordion } from "flowbite-react";

export default function AccessBox() {
  return (
    <>
      <div>
        <Accordion
          collapseAll
          className="mb-6  border border-[#238880] rounded-3xl"
        >
          <Accordion.Panel>
            <div className="">
              <Accordion.Title className=" bg-[#238880] rounded-3xl first:rounded-t-3xl  text-white focus:ring-0 focus:text-black  hover:text-black">
                <table>
                  <tbody>
                    <tr>
                      <td className="max-h-8 w-[200px]">Access Control</td>
                    </tr>
                  </tbody>
                </table>
              </Accordion.Title>
              <Accordion.Content>
                <div className="flex flex-col gap-3">
                  <div className="flex max-w-fit flex-row gap-3">
                    <input
                      type="radio"
                      id="ownable"
                      name="access"
                      placeholder=""
                      className="checked:bg-[#238880]"
                    />
                    <label className="text-label text-sm text-left">
                      Ownable
                    </label>
                  </div>
                  <div className="flex max-w-fit flex-row gap-3">
                    <input
                      type="radio"
                      id="roles"
                      name="access"
                      placeholder=""
                      className="checked:bg-[#238880]"
                    />
                    <label className="text-label text-sm text-left">
                      Roles
                    </label>
                  </div>
                </div>
              </Accordion.Content>
            </div>
          </Accordion.Panel>
        </Accordion>
      </div>
    </>
  );
}
