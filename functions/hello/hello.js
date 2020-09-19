const { hello } = require("./native/index.node");

exports.handler = async function ({ queryStringParameters = {} }) {
  try {
    const { name = "World" } = queryStringParameters;
    const message = hello(name);

    return {
      statusCode: 200,
      body: message,
    };
  } catch (error) {
    console.log(error);
    return { statusCode: error.statusCode || 500, body: error.message };
  }
};
