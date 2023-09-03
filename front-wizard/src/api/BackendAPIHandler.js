const getApiUrl = (ep) => {
  return import.meta.env.VITE_BACKEND_URL + ep;
};

const prepareRequestBody = (bodyContent) => {
  return {
    method: "POST",
    headers: {
      "Content-Type": "application/json", // Set the content type to JSON
    },
    body: JSON.stringify(bodyContent),
  };
};

export function sendGenerateRequest(standardData, featuresData) {
  // Create the JSON object with the desired fields
  const requestBody = {
    standard: standardData,
    features: featuresData,
  };

  // Make the POST request
  return fetch(
    getApiUrl(import.meta.env.VITE_GENERATE_ENDPOINT),
    prepareRequestBody(requestBody),
  )
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json(); // Parse the response as JSON
    })
    .then((data) => {
      // Handle the API response data here
      console.log(data);
      return data; // You can return the data or perform further operations
    })
    .catch((error) => {
      console.error("Request failed:", error);
      throw error;
    });
}

export function sendBuildRequest(standardData, featuresData) {
  const requestBody = {
    standard: standardData,
    features: featuresData,
  };

  return fetch(
    getApiUrl(import.meta.env.VITE_BUILD_ENDPOINT),
    prepareRequestBody(requestBody),
  )
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return response.json(); // Parse the response as JSON
    })
    .catch((error) => {
      console.error("Request failed:", error);
      throw error;
    });
}
