export default class ApiError extends Error {

  statusCode: number;

  constructor(statusCode: number) {
    super();
    this.statusCode = statusCode;
  }

}