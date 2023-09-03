import PropTypes from "prop-types";

const FormCheckbox = ({
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
  FormCheckbox.propTypes = {
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

  return (
    <div className="flex max-w-fit flex-row gap-3">
      <input
        type="checkbox"
        id={name}
        placeholder={placeholder || ""}
        className={`${
          disabled
            ? "!bg-grey-50 !border-grey-200 cursor-not-allowed"
            : "bg-white text-black"
        } block rounded border-[1px] px-1 py-1.5 text-base focus:!outline-primary-100 focus:!outline focus:!outline-4 checked:bg-[#238880] ${
          error ? "!border-error" : ""
        } ${className || ""}`}
        {...props}
        {...field}
        disabled={disabled}
        onChange={formik?.handleChange}
      />
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
    </div>
  );
};

export default FormCheckbox;
