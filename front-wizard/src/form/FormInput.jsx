import PropTypes from "prop-types";
import { useDebouncedCallback } from "use-debounce";
import {useFormikContext} from "formik";

const FormInput = ({
  formik,
  className,
  errors,
  label,
  name,
  placeholder,
  disabled = false,
  field,
  ...props
}) => {
  const error = errors && errors[name];
  FormInput.propTypes = {
    className: PropTypes.string,
    errors: PropTypes.object,
    isSubmitted: PropTypes.bool,
    label: PropTypes.string,
    name: PropTypes.string.isRequired,
    placeholder: PropTypes.string,
    register: PropTypes.func,
    rules: PropTypes.object,
    disabled: PropTypes.bool,
    field: PropTypes.object,
  };
  const debounced = useDebouncedCallback(
    (field, value, e) => {
      // formik.setFieldValue(field, value, true);
      console.log("Debounced");
      formik.handleChange(e)
      // formik.

      setValues(formik.values())
    },
    1000,
  );

  return (
    <div className="flex max-w-fit flex-col gap-1">
      {label ? (
        <label
          htmlFor={name}
          className={`text-label text-sm text-left ${
            error ? "text-error" : ""
          }`}
        >
          {label}
        </label>
      ) : null}
      <input
        id={name}
        name={name}
        placeholder={placeholder || ""}
        className={`${
          disabled
            ? "!bg-grey-50 !border-grey-200 cursor-not-allowed"
            : "bg-white text-black"
        } block rounded border-[1px] px-1 py-1.5 text-base focus:!outline-primary-100 focus:!outline focus:!outline-4 ${
          error ? "!border-error" : ""
        } ${className || ""}`}
        {...props}
        {...field}
        disabled={disabled}
        onChange={(e) => {
          console.log("e: ", e)
          const { name, value } = e.target;
          debounced(name, value, e);
        }}
      />
    </div>
  );
};

export default FormInput;
