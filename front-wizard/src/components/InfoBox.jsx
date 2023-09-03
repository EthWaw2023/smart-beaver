import { Accordion } from "flowbite-react";
import FormInput from "../form/FormInput";

export default function InfoBox() {
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
                      <td className="max-h-8 w-[200px]">Info</td>
                    </tr>
                  </tbody>
                </table>
              </Accordion.Title>
              <Accordion.Content>
                <div className="flex flex-col gap-3 items-center">
                  <FormInput name="security" label="Security Contact" />
                  <FormInput name="license" label="License" />
                </div>
              </Accordion.Content>
            </div>
          </Accordion.Panel>
        </Accordion>
      </div>
    </>
  );
}
