import { Accordion } from "flowbite-react";
import FormCheckbox from "../form/FormCheckbox";

export default function FeaturesBoxPSP37({formik}) {
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
                      <td className="max-h-8 w-[200px]">Features</td>
                    </tr>
                  </tbody>
                </table>
              </Accordion.Title>
              <Accordion.Content>
                <div className="flex flex-col gap-3">
                  <FormCheckbox formik={formik} name="batch" label="Batch" />
                  <FormCheckbox formik={formik} name="metadata" label="Metadata" />
                  <FormCheckbox formik={formik} name="mintable" label="Mintable" />
                  <FormCheckbox formik={formik} name="burnable" label="Burnable" />
                  <FormCheckbox formik={formik} name="enumerable" label="Enumerable" />
                </div>
              </Accordion.Content>
            </div>
          </Accordion.Panel>
        </Accordion>
      </div>
    </>
  );
}
