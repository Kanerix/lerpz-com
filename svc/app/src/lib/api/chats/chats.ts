// @ts-nocheck
import {
  createMutation,
  createQuery
} from '@tanstack/svelte-query';
import type {
  CreateMutationOptions,
  CreateMutationResult,
  CreateQueryOptions,
  CreateQueryResult,
  DataTag,
  MutationFunction,
  QueryClient,
  QueryFunction,
  QueryKey
} from '@tanstack/svelte-query';

import type {
  ChatRequest,
  ConversationDetailResponse,
  ConversationResponse,
  EditLatestMessageRequest,
  MessageRequest,
  ProblemSchema,
  UpdateChatRequest
} from '../models';

import { customFetch } from '../../http/orval-mutator';
import type { ErrorType } from '../../http/orval-mutator';


type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];



export type listChatsResponse200 = {
  data: ConversationResponse[]
  status: 200
}

export type listChatsResponse401 = {
  data: ProblemSchema
  status: 401
}

export type listChatsResponse500 = {
  data: ProblemSchema
  status: 500
}

export type listChatsResponseSuccess = (listChatsResponse200) & {
  headers: Headers;
};
export type listChatsResponseError = (listChatsResponse401 | listChatsResponse500) & {
  headers: Headers;
};

export type listChatsResponse = (listChatsResponseSuccess | listChatsResponseError)

export const getListChatsUrl = () => {




  return `/api/v1/chats`
}

/**
 * Returns a list of the authenticated user's conversations ordered by most recently updated.
 * @summary Get a list of chats
 */
export const listChats = async ( options?: RequestInit): Promise<listChatsResponse> => {

  return customFetch<listChatsResponse>(getListChatsUrl(),
  {
    ...options,
    method: 'GET'


  }
);}





export const getListChatsQueryKey = () => {
    return [
    `/api/v1/chats`
    ] as const;
    }


export const getListChatsQueryOptions = <TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>( options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getListChatsQueryKey();



    const queryFn: QueryFunction<Awaited<ReturnType<typeof listChats>>> = ({ signal }) => listChats({ signal, ...requestOptions });





   return  { queryKey, queryFn, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type ListChatsQueryResult = NonNullable<Awaited<ReturnType<typeof listChats>>>
export type ListChatsQueryError = ErrorType<ProblemSchema>


/**
 * @summary Get a list of chats
 */

export function createListChats<TData = Awaited<ReturnType<typeof listChats>>, TError = ErrorType<ProblemSchema>>(
  options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof listChats>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getListChatsQueryOptions(options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type createChatResponse200 = {
  data: string
  status: 200
}

export type createChatResponse400 = {
  data: ProblemSchema
  status: 400
}

export type createChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type createChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type createChatResponseSuccess = (createChatResponse200) & {
  headers: Headers;
};
export type createChatResponseError = (createChatResponse400 | createChatResponse401 | createChatResponse500) & {
  headers: Headers;
};

export type createChatResponse = (createChatResponseSuccess | createChatResponseError)

export const getCreateChatUrl = () => {




  return `/api/v1/chats`
}

/**
 * Creates a new conversation and streams the AI reply via Server-Sent Events.
 *
 * Events:
 * - `conversation_created`: new conversation UUID, sent first
 * - `reasoning`: chain-of-thought chunk, reasoning models only
 * - `message`: answer token chunk
 * - `saved`: conversation UUID confirming persistence, sent last
 * - `error`: problem document describing the failure
 * @summary Create a new chat
 */
export const createChat = async (chatRequest: ChatRequest, options?: RequestInit): Promise<createChatResponse> => {

  return customFetch<createChatResponse>(getCreateChatUrl(),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(chatRequest)
  }
);}




export const getCreateChatMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext> => {

const mutationKey = ['createChat'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof createChat>>, {data: ChatRequest}> = (props) => {
          const {data} = props ?? {};

          return  createChat(data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type CreateChatMutationResult = NonNullable<Awaited<ReturnType<typeof createChat>>>
    export type CreateChatMutationBody = ChatRequest
    export type CreateChatMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Create a new chat
 */
export const createCreateChat = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof createChat>>, TError,{data: ChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof createChat>>,
        TError,
        {data: ChatRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getCreateChatMutationOptions(options?.()) }), queryClient);
    }
    export type getChatResponse200 = {
  data: ConversationDetailResponse
  status: 200
}

export type getChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type getChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type getChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type getChatResponseSuccess = (getChatResponse200) & {
  headers: Headers;
};
export type getChatResponseError = (getChatResponse401 | getChatResponse404 | getChatResponse500) & {
  headers: Headers;
};

export type getChatResponse = (getChatResponseSuccess | getChatResponseError)

export const getGetChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Returns a conversation and all its messages for the authenticated user.
 * @summary Get a specific chat
 */
export const getChat = async (id: string, options?: RequestInit): Promise<getChatResponse> => {

  return customFetch<getChatResponse>(getGetChatUrl(id),
  {
    ...options,
    method: 'GET'


  }
);}





export const getGetChatQueryKey = (id: string,) => {
    return [
    `/api/v1/chats/${id}`
    ] as const;
    }


export const getGetChatQueryOptions = <TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(id: string, options?: { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
) => {

const {query: queryOptions, request: requestOptions} = options ?? {};

  const queryKey =  queryOptions?.queryKey ?? getGetChatQueryKey(id);



    const queryFn: QueryFunction<Awaited<ReturnType<typeof getChat>>> = ({ signal }) => getChat(id, { signal, ...requestOptions });





   return  { queryKey, queryFn, enabled: id !== null && id !== undefined, ...queryOptions} as CreateQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData> & { queryKey: DataTag<QueryKey, TData, TError> }
}

export type GetChatQueryResult = NonNullable<Awaited<ReturnType<typeof getChat>>>
export type GetChatQueryError = ErrorType<ProblemSchema>


/**
 * @summary Get a specific chat
 */

export function createGetChat<TData = Awaited<ReturnType<typeof getChat>>, TError = ErrorType<ProblemSchema>>(
 id: () =>  string, options?: () => { query?:Partial<CreateQueryOptions<Awaited<ReturnType<typeof getChat>>, TError, TData>>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient
 ): CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> } {



  const query = createQuery(() => getGetChatQueryOptions(id(),options?.()), queryClient) as CreateQueryResult<TData, TError> & { queryKey: DataTag<QueryKey, TData, TError> };

  return query
}






export type sendChatMessageResponse200 = {
  data: string
  status: 200
}

export type sendChatMessageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type sendChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type sendChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type sendChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type sendChatMessageResponseSuccess = (sendChatMessageResponse200) & {
  headers: Headers;
};
export type sendChatMessageResponseError = (sendChatMessageResponse400 | sendChatMessageResponse401 | sendChatMessageResponse404 | sendChatMessageResponse500) & {
  headers: Headers;
};

export type sendChatMessageResponse = (sendChatMessageResponseSuccess | sendChatMessageResponseError)

export const getSendChatMessageUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Appends a new user message to the conversation and streams the AI reply via Server-Sent Events. Requires the conversation to belong to the authenticated user.
 *
 * Events:
 * - `reasoning`: chain-of-thought chunk, reasoning models only
 * - `message`: answer token chunk
 * - `saved`: conversation UUID confirming persistence, sent last
 * - `error`: problem document describing the failure
 * @summary Send a message in an existing chat
 */
export const sendChatMessage = async (id: string,
    messageRequest: MessageRequest, options?: RequestInit): Promise<sendChatMessageResponse> => {

  return customFetch<sendChatMessageResponse>(getSendChatMessageUrl(id),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(messageRequest)
  }
);}




export const getSendChatMessageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext> => {

const mutationKey = ['sendChatMessage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof sendChatMessage>>, {id: string;data: MessageRequest}> = (props) => {
          const {id,data} = props ?? {};

          return  sendChatMessage(id,data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type SendChatMessageMutationResult = NonNullable<Awaited<ReturnType<typeof sendChatMessage>>>
    export type SendChatMessageMutationBody = MessageRequest
    export type SendChatMessageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Send a message in an existing chat
 */
export const createSendChatMessage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof sendChatMessage>>, TError,{id: string;data: MessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof sendChatMessage>>,
        TError,
        {id: string;data: MessageRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getSendChatMessageMutationOptions(options?.()) }), queryClient);
    }
    export type deleteChatResponse204 = {
  data: void
  status: 204
}

export type deleteChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteChatResponseSuccess = (deleteChatResponse204) & {
  headers: Headers;
};
export type deleteChatResponseError = (deleteChatResponse401 | deleteChatResponse404 | deleteChatResponse500) & {
  headers: Headers;
};

export type deleteChatResponse = (deleteChatResponseSuccess | deleteChatResponseError)

export const getDeleteChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Permanently deletes a conversation and all of its messages. Requires the conversation to belong to the authenticated user.
 * @summary Delete a chat
 */
export const deleteChat = async (id: string, options?: RequestInit): Promise<deleteChatResponse> => {

  return customFetch<deleteChatResponse>(getDeleteChatUrl(id),
  {
    ...options,
    method: 'DELETE'


  }
);}




export const getDeleteChatMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteChat>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof deleteChat>>, TError,{id: string}, TContext> => {

const mutationKey = ['deleteChat'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteChat>>, {id: string}> = (props) => {
          const {id} = props ?? {};

          return  deleteChat(id,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteChatMutationResult = NonNullable<Awaited<ReturnType<typeof deleteChat>>>

    export type DeleteChatMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a chat
 */
export const createDeleteChat = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteChat>>, TError,{id: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof deleteChat>>,
        TError,
        {id: string},
        TContext
      > => {
      return createMutation(() => ({ ...getDeleteChatMutationOptions(options?.()) }), queryClient);
    }
    export type updateChatResponse200 = {
  data: ConversationResponse
  status: 200
}

export type updateChatResponse401 = {
  data: ProblemSchema
  status: 401
}

export type updateChatResponse404 = {
  data: ProblemSchema
  status: 404
}

export type updateChatResponse500 = {
  data: ProblemSchema
  status: 500
}

export type updateChatResponseSuccess = (updateChatResponse200) & {
  headers: Headers;
};
export type updateChatResponseError = (updateChatResponse401 | updateChatResponse404 | updateChatResponse500) & {
  headers: Headers;
};

export type updateChatResponse = (updateChatResponseSuccess | updateChatResponseError)

export const getUpdateChatUrl = (id: string,) => {




  return `/api/v1/chats/${id}`
}

/**
 * Updates mutable fields on a conversation owned by the authenticated user. Currently supports archiving and unarchiving.
 * @summary Update a chat
 */
export const updateChat = async (id: string,
    updateChatRequest: UpdateChatRequest, options?: RequestInit): Promise<updateChatResponse> => {

  return customFetch<updateChatResponse>(getUpdateChatUrl(id),
  {
    ...options,
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(updateChatRequest)
  }
);}




export const getUpdateChatMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof updateChat>>, TError,{id: string;data: UpdateChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof updateChat>>, TError,{id: string;data: UpdateChatRequest}, TContext> => {

const mutationKey = ['updateChat'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof updateChat>>, {id: string;data: UpdateChatRequest}> = (props) => {
          const {id,data} = props ?? {};

          return  updateChat(id,data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type UpdateChatMutationResult = NonNullable<Awaited<ReturnType<typeof updateChat>>>
    export type UpdateChatMutationBody = UpdateChatRequest
    export type UpdateChatMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Update a chat
 */
export const createUpdateChat = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof updateChat>>, TError,{id: string;data: UpdateChatRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof updateChat>>,
        TError,
        {id: string;data: UpdateChatRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getUpdateChatMutationOptions(options?.()) }), queryClient);
    }
    export type editLatestChatMessageResponse200 = {
  data: string
  status: 200
}

export type editLatestChatMessageResponse400 = {
  data: ProblemSchema
  status: 400
}

export type editLatestChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type editLatestChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type editLatestChatMessageResponse409 = {
  data: ProblemSchema
  status: 409
}

export type editLatestChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type editLatestChatMessageResponseSuccess = (editLatestChatMessageResponse200) & {
  headers: Headers;
};
export type editLatestChatMessageResponseError = (editLatestChatMessageResponse400 | editLatestChatMessageResponse401 | editLatestChatMessageResponse404 | editLatestChatMessageResponse409 | editLatestChatMessageResponse500) & {
  headers: Headers;
};

export type editLatestChatMessageResponse = (editLatestChatMessageResponseSuccess | editLatestChatMessageResponseError)

export const getEditLatestChatMessageUrl = (id: string,) => {




  return `/api/v1/chats/${id}/messages/latest`
}

/**
 * Replaces the content of the conversation's most recent user message, discards the assistant reply (and any later turns) that followed it, then regenerates and streams a fresh reply via Server-Sent Events. Only the latest message can be edited, since editing an earlier one would require regenerating everything after it.
 *
 * Events:
 * - `reasoning`: chain-of-thought chunk, reasoning models only
 * - `message`: answer token chunk
 * - `saved`: conversation UUID confirming persistence, sent last
 * - `error`: problem document describing the failure
 * @summary Edit the latest message in a chat
 */
export const editLatestChatMessage = async (id: string,
    editLatestMessageRequest: EditLatestMessageRequest, options?: RequestInit): Promise<editLatestChatMessageResponse> => {

  return customFetch<editLatestChatMessageResponse>(getEditLatestChatMessageUrl(id),
  {
    ...options,
    method: 'POST',
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    body: JSON.stringify(editLatestMessageRequest)
  }
);}




export const getEditLatestChatMessageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError,{id: string;data: EditLatestMessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError,{id: string;data: EditLatestMessageRequest}, TContext> => {

const mutationKey = ['editLatestChatMessage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof editLatestChatMessage>>, {id: string;data: EditLatestMessageRequest}> = (props) => {
          const {id,data} = props ?? {};

          return  editLatestChatMessage(id,data,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type EditLatestChatMessageMutationResult = NonNullable<Awaited<ReturnType<typeof editLatestChatMessage>>>
    export type EditLatestChatMessageMutationBody = EditLatestMessageRequest
    export type EditLatestChatMessageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Edit the latest message in a chat
 */
export const createEditLatestChatMessage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof editLatestChatMessage>>, TError,{id: string;data: EditLatestMessageRequest}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof editLatestChatMessage>>,
        TError,
        {id: string;data: EditLatestMessageRequest},
        TContext
      > => {
      return createMutation(() => ({ ...getEditLatestChatMessageMutationOptions(options?.()) }), queryClient);
    }
    export type deleteChatMessageResponse204 = {
  data: void
  status: 204
}

export type deleteChatMessageResponse401 = {
  data: ProblemSchema
  status: 401
}

export type deleteChatMessageResponse404 = {
  data: ProblemSchema
  status: 404
}

export type deleteChatMessageResponse500 = {
  data: ProblemSchema
  status: 500
}

export type deleteChatMessageResponseSuccess = (deleteChatMessageResponse204) & {
  headers: Headers;
};
export type deleteChatMessageResponseError = (deleteChatMessageResponse401 | deleteChatMessageResponse404 | deleteChatMessageResponse500) & {
  headers: Headers;
};

export type deleteChatMessageResponse = (deleteChatMessageResponseSuccess | deleteChatMessageResponseError)

export const getDeleteChatMessageUrl = (id: string,
    messageId: string,) => {




  return `/api/v1/chats/${id}/messages/${messageId}`
}

/**
 * Permanently deletes the given message and all messages that follow it in the conversation, keeping every earlier message intact. Requires the conversation to belong to the authenticated user.
 * @summary Delete a message and everything after it
 */
export const deleteChatMessage = async (id: string,
    messageId: string, options?: RequestInit): Promise<deleteChatMessageResponse> => {

  return customFetch<deleteChatMessageResponse>(getDeleteChatMessageUrl(id,messageId),
  {
    ...options,
    method: 'DELETE'


  }
);}




export const getDeleteChatMessageMutationOptions = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError,{id: string;messageId: string}, TContext>, request?: SecondParameter<typeof customFetch>}
): CreateMutationOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError,{id: string;messageId: string}, TContext> => {

const mutationKey = ['deleteChatMessage'];
const {mutation: mutationOptions, request: requestOptions} = options ?
      options.mutation && 'mutationKey' in options.mutation && options.mutation.mutationKey ?
      options
      : {...options, mutation: {...options.mutation, mutationKey}}
      : {mutation: { mutationKey, }, request: undefined};




      const mutationFn: MutationFunction<Awaited<ReturnType<typeof deleteChatMessage>>, {id: string;messageId: string}> = (props) => {
          const {id,messageId} = props ?? {};

          return  deleteChatMessage(id,messageId,requestOptions)
        }






  return  { mutationFn, ...mutationOptions }}

    export type DeleteChatMessageMutationResult = NonNullable<Awaited<ReturnType<typeof deleteChatMessage>>>

    export type DeleteChatMessageMutationError = ErrorType<ProblemSchema>

    /**
 * @summary Delete a message and everything after it
 */
export const createDeleteChatMessage = <TError = ErrorType<ProblemSchema>,
    TContext = unknown>(options?: () => { mutation?:CreateMutationOptions<Awaited<ReturnType<typeof deleteChatMessage>>, TError,{id: string;messageId: string}, TContext>, request?: SecondParameter<typeof customFetch>}
 , queryClient?: () => QueryClient): CreateMutationResult<
        Awaited<ReturnType<typeof deleteChatMessage>>,
        TError,
        {id: string;messageId: string},
        TContext
      > => {
      return createMutation(() => ({ ...getDeleteChatMessageMutationOptions(options?.()) }), queryClient);
    }
