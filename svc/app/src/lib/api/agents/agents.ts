// @ts-nocheck
import type {
  AgentMemoryResponse,
  AgentResponse,
  CreateAgentRequest,
  ProblemSchema
} from '../models';

import { agentFetch } from '../../http/agent-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type listAgentMemoryResponse200 = {
  data: AgentMemoryResponse[]
  status: 200
}

export type listAgentMemoryResponse400 = {
  data: ProblemSchema
  status: 400
}

export type listAgentMemoryResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listAgentMemoryResponse403 = {
  data: ProblemSchema
  status: 403
}

export type listAgentMemoryResponse404 = {
  data: ProblemSchema
  status: 404
}

export type listAgentMemoryResponse409 = {
  data: ProblemSchema
  status: 409
}

export type listAgentMemoryResponse502 = {
  data: ProblemSchema
  status: 502
}

export type listAgentMemoryResponse504 = {
  data: ProblemSchema
  status: 504
}

export type listAgentMemoryResponseSuccess = (listAgentMemoryResponse200) & {
  headers: Headers;
};
export type listAgentMemoryResponseError = (listAgentMemoryResponse400 | listAgentMemoryResponse401 | listAgentMemoryResponse403 | listAgentMemoryResponse404 | listAgentMemoryResponse409 | listAgentMemoryResponse502 | listAgentMemoryResponse504) & {
  headers: Headers;
};

export type listAgentMemoryResponse = (listAgentMemoryResponseSuccess | listAgentMemoryResponseError)

export const getListAgentMemoryUrl = () => {




  return `/api/v1/agent-memory`
}

/**
 * Lists the caller's agent memory, including memory retained after an agent is deleted. Each item reports its availability and whether it is in use.
 * @summary List agent memory
 */
export const listAgentMemory = async ( options?: RequestInit): Promise<listAgentMemoryResponse> => {

  return agentFetch<listAgentMemoryResponse>(getListAgentMemoryUrl(),
  {
    ...options,
    method: 'GET'


  }
);}



export type readAgentMemoryResponse200 = {
  data: AgentMemoryResponse
  status: 200
}

export type readAgentMemoryResponse400 = {
  data: ProblemSchema
  status: 400
}

export type readAgentMemoryResponse401 = {
  data: ProblemSchema
  status: 401
}

export type readAgentMemoryResponse403 = {
  data: ProblemSchema
  status: 403
}

export type readAgentMemoryResponse404 = {
  data: ProblemSchema
  status: 404
}

export type readAgentMemoryResponse409 = {
  data: ProblemSchema
  status: 409
}

export type readAgentMemoryResponse502 = {
  data: ProblemSchema
  status: 502
}

export type readAgentMemoryResponse504 = {
  data: ProblemSchema
  status: 504
}

export type readAgentMemoryResponseSuccess = (readAgentMemoryResponse200) & {
  headers: Headers;
};
export type readAgentMemoryResponseError = (readAgentMemoryResponse400 | readAgentMemoryResponse401 | readAgentMemoryResponse403 | readAgentMemoryResponse404 | readAgentMemoryResponse409 | readAgentMemoryResponse502 | readAgentMemoryResponse504) & {
  headers: Headers;
};

export type readAgentMemoryResponse = (readAgentMemoryResponseSuccess | readAgentMemoryResponseError)

export const getReadAgentMemoryUrl = (name: string,) => {




  return `/api/v1/agent-memory/${name}`
}

/**
 * Returns the caller's memory for the named agent, including after the agent is deleted. Reports its availability and whether it is in use. Missing or inaccessible memory returns 404.
 * @summary Get agent memory
 */
export const readAgentMemory = async (name: string, options?: RequestInit): Promise<readAgentMemoryResponse> => {

  return agentFetch<readAgentMemoryResponse>(getReadAgentMemoryUrl(name),
  {
    ...options,
    method: 'GET'


  }
);}



export type deleteAgentMemoryResponse204 = {
  data: void
  status: 204
}

export type deleteAgentMemoryResponse400 = {
  data: ProblemSchema
  status: 400
}

export type deleteAgentMemoryResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteAgentMemoryResponse403 = {
  data: ProblemSchema
  status: 403
}

export type deleteAgentMemoryResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteAgentMemoryResponse409 = {
  data: ProblemSchema
  status: 409
}

export type deleteAgentMemoryResponse502 = {
  data: ProblemSchema
  status: 502
}

export type deleteAgentMemoryResponse504 = {
  data: ProblemSchema
  status: 504
}

export type deleteAgentMemoryResponseSuccess = (deleteAgentMemoryResponse204) & {
  headers: Headers;
};
export type deleteAgentMemoryResponseError = (deleteAgentMemoryResponse400 | deleteAgentMemoryResponse401 | deleteAgentMemoryResponse403 | deleteAgentMemoryResponse404 | deleteAgentMemoryResponse409 | deleteAgentMemoryResponse502 | deleteAgentMemoryResponse504) & {
  headers: Headers;
};

export type deleteAgentMemoryResponse = (deleteAgentMemoryResponseSuccess | deleteAgentMemoryResponseError)

export const getDeleteAgentMemoryUrl = (name: string,) => {




  return `/api/v1/agent-memory/${name}`
}

/**
 * Requests removal of the caller's memory for the named agent. Memory mounted by the matching agent cannot be removed. Missing or inaccessible memory returns 404.
 * @summary Remove agent memory
 */
export const deleteAgentMemory = async (name: string, options?: RequestInit): Promise<deleteAgentMemoryResponse> => {

  return agentFetch<deleteAgentMemoryResponse>(getDeleteAgentMemoryUrl(name),
  {
    ...options,
    method: 'DELETE'


  }
);}



export type listAgentsResponse200 = {
  data: AgentResponse[]
  status: 200
}

export type listAgentsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listAgentsResponse403 = {
  data: ProblemSchema
  status: 403
}

export type listAgentsResponse502 = {
  data: ProblemSchema
  status: 502
}

export type listAgentsResponse504 = {
  data: ProblemSchema
  status: 504
}

export type listAgentsResponseSuccess = (listAgentsResponse200) & {
  headers: Headers;
};
export type listAgentsResponseError = (listAgentsResponse401 | listAgentsResponse403 | listAgentsResponse502 | listAgentsResponse504) & {
  headers: Headers;
};

export type listAgentsResponse = (listAgentsResponseSuccess | listAgentsResponseError)

export const getListAgentsUrl = () => {




  return `/api/v1/agents`
}

/**
 * Lists only agents owned by the authenticated user. Retained memory is listed separately.
 * @summary List private agents
 */
export const listAgents = async ( options?: RequestInit): Promise<listAgentsResponse> => {

  return agentFetch<listAgentsResponse>(getListAgentsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}



export type createAgentResponse201 = {
  data: AgentResponse
  status: 201
}

export type createAgentResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createAgentResponse403 = {
  data: ProblemSchema
  status: 403
}

export type createAgentResponse404 = {
  data: ProblemSchema
  status: 404
}

export type createAgentResponse409 = {
  data: ProblemSchema
  status: 409
}

export type createAgentResponse422 = {
  data: ProblemSchema
  status: 422
}

export type createAgentResponse502 = {
  data: ProblemSchema
  status: 502
}

export type createAgentResponse504 = {
  data: ProblemSchema
  status: 504
}

export type createAgentResponseSuccess = (createAgentResponse201) & {
  headers: Headers;
};
export type createAgentResponseError = (createAgentResponse400 | createAgentResponse401 | createAgentResponse403 | createAgentResponse404 | createAgentResponse409 | createAgentResponse422 | createAgentResponse502 | createAgentResponse504) & {
  headers: Headers;
};

export type createAgentResponse = (createAgentResponseSuccess | createAgentResponseError)

export const getCreateAgentUrl = () => {




  return `/api/v1/agents`
}

/**
 * Creates an agent and prepares its selected memory. Existing memory must belong to the caller and have the same agent name. New memory uses platform storage settings. A successful response does not mean the agent is ready yet. Creation is not retried automatically. If startup fails, memory is kept; check the agent and memory before retrying.
 * @summary Create a private agent
 */
export const createAgent = async (createAgentRequest: CreateAgentRequest, options?: RequestInit): Promise<createAgentResponse> => {

  return agentFetch<createAgentResponse>(getCreateAgentUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(createAgentRequest)
  }
);}



export type readAgentResponse200 = {
  data: AgentResponse
  status: 200
}

export type readAgentResponse400 = {
  data: ProblemSchema
  status: 400
}

export type readAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type readAgentResponse403 = {
  data: ProblemSchema
  status: 403
}

export type readAgentResponse404 = {
  data: ProblemSchema
  status: 404
}

export type readAgentResponse502 = {
  data: ProblemSchema
  status: 502
}

export type readAgentResponse504 = {
  data: ProblemSchema
  status: 504
}

export type readAgentResponseSuccess = (readAgentResponse200) & {
  headers: Headers;
};
export type readAgentResponseError = (readAgentResponse400 | readAgentResponse401 | readAgentResponse403 | readAgentResponse404 | readAgentResponse502 | readAgentResponse504) & {
  headers: Headers;
};

export type readAgentResponse = (readAgentResponseSuccess | readAgentResponseError)

export const getReadAgentUrl = (name: string,) => {




  return `/api/v1/agents/${name}`
}

/**
 * Returns an agent owned by the authenticated user. Missing and inaccessible agents return 404.
 * @summary Read a private agent
 */
export const readAgent = async (name: string, options?: RequestInit): Promise<readAgentResponse> => {

  return agentFetch<readAgentResponse>(getReadAgentUrl(name),
  {
    ...options,
    method: 'GET'


  }
);}



export type deleteAgentResponse204 = {
  data: void
  status: 204
}

export type deleteAgentResponse400 = {
  data: ProblemSchema
  status: 400
}

export type deleteAgentResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteAgentResponse403 = {
  data: ProblemSchema
  status: 403
}

export type deleteAgentResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteAgentResponse409 = {
  data: ProblemSchema
  status: 409
}

export type deleteAgentResponse502 = {
  data: ProblemSchema
  status: 502
}

export type deleteAgentResponse504 = {
  data: ProblemSchema
  status: 504
}

export type deleteAgentResponseSuccess = (deleteAgentResponse204) & {
  headers: Headers;
};
export type deleteAgentResponseError = (deleteAgentResponse400 | deleteAgentResponse401 | deleteAgentResponse403 | deleteAgentResponse404 | deleteAgentResponse409 | deleteAgentResponse502 | deleteAgentResponse504) & {
  headers: Headers;
};

export type deleteAgentResponse = (deleteAgentResponseSuccess | deleteAgentResponseError)

export const getDeleteAgentUrl = (name: string,) => {




  return `/api/v1/agents/${name}`
}

/**
 * Requests removal of an agent owned by the authenticated user. Its persistent memory is kept and can be reused by a new agent with the same name. Removal may take a moment.
 * @summary Delete a private agent
 */
export const deleteAgent = async (name: string, options?: RequestInit): Promise<deleteAgentResponse> => {

  return agentFetch<deleteAgentResponse>(getDeleteAgentUrl(name),
  {
    ...options,
    method: 'DELETE'


  }
);}



