import {
  ExceptionFilter,
  Catch,
  ArgumentsHost,
  HttpException,
  Logger
} from '@nestjs/common';

@Catch()
export class AllExceptionsFilter implements ExceptionFilter {
  private readonly logger = new Logger(AllExceptionsFilter.name);

  catch(exception: unknown, host: ArgumentsHost) {
    if (exception instanceof AggregateError) {
      this.logger.error(
        `AggregateError: ${exception.message}`,
        exception.errors.map((e, i) => `  [${i}] ${e.stack || e}`).join('\n')
      );
    } else if (exception instanceof Error) {
      this.logger.error(exception.stack || exception.message);
    } else {
      this.logger.error('Unknown exception', JSON.stringify(exception));
    }

    const ctx = host.switchToHttp();
    const response = ctx.getResponse();
    const status =
      exception instanceof HttpException ? exception.getStatus() : 500;
    const message =
      exception instanceof HttpException
        ? exception.getResponse()
        : 'Internal server error';

    response.status(status).json({
      statusCode: status,
      error: message,
    });
  }
}
