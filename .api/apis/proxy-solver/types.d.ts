import type { FromSchema } from 'json-schema-to-ts';
import * as schemas from './schemas';
export type PostSolutionBodyParam = FromSchema<typeof schemas.PostSolution.body>;
export type PostSolutionResponse200 = FromSchema<typeof schemas.PostSolution.response['200']>;
