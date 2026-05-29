// @ts-nocheck

import type {
    CreateMutationOptions,
    CreateMutationResult,
    MutationFunction,
    QueryClient,
} from "@tanstack/svelte-query";
import { useMutation } from "@tanstack/svelte-query";
import type { ErrorType } from "$lib/http/orval-mutator.js";

import { customFetch } from "$lib/http/orval-mutator.js";
import type { ProblemSchema } from "../models/index.js";

type SecondParameter<T extends (...args: never) => unknown> = Parameters<T>[1];

/**
 * @summary Create a new image
 */
export type createImageResponse400 = {
    data: ProblemSchema;
    status: 400;
};

export type createImageResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type createImageResponse500 = {
    data: ProblemSchema;
    status: 500;
};
export type createImageResponseError = (
    | createImageResponse400
    | createImageResponse401
    | createImageResponse500
) & {
    headers: Headers;
};

export type createImageResponse = createImageResponseError;

export const getCreateImageUrl = () => {
    return `/api/v1/images`;
};

export const createImage = async (
    options?: RequestInit,
): Promise<createImageResponse> => {
    return customFetch<createImageResponse>(getCreateImageUrl(), {
        ...options,
        method: "POST",
    });
};

export const getCreateImageMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof createImage>>,
        TError,
        void,
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof createImage>>,
    TError,
    void,
    TContext
> => {
    const mutationKey = ["createImage"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof createImage>>,
        void
    > = () => {
        return createImage(requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type CreateImageMutationResult = NonNullable<
    Awaited<ReturnType<typeof createImage>>
>;

export type CreateImageMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Create a new image
 */
export const useCreateImage = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof createImage>>,
            TError,
            void,
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof createImage>>,
    TError,
    void,
    TContext
> => {
    return useMutation(getCreateImageMutationOptions(options), queryClient);
};
/**
 * @summary Create a new image from existing images
 */
export type editImageResponse200 = {
    data: undefined;
    status: 200;
};

export type editImageResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type editImageResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type editImageResponseSuccess = editImageResponse200 & {
    headers: Headers;
};
export type editImageResponseError = (
    | editImageResponse401
    | editImageResponse500
) & {
    headers: Headers;
};

export type editImageResponse =
    | editImageResponseSuccess
    | editImageResponseError;

export const getEditImageUrl = () => {
    return `/api/v1/images/edit`;
};

export const editImage = async (
    options?: RequestInit,
): Promise<editImageResponse> => {
    return customFetch<editImageResponse>(getEditImageUrl(), {
        ...options,
        method: "POST",
    });
};

export const getEditImageMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof editImage>>,
        TError,
        void,
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof editImage>>,
    TError,
    void,
    TContext
> => {
    const mutationKey = ["editImage"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof editImage>>,
        void
    > = () => {
        return editImage(requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type EditImageMutationResult = NonNullable<
    Awaited<ReturnType<typeof editImage>>
>;

export type EditImageMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Create a new image from existing images
 */
export const useEditImage = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof editImage>>,
            TError,
            void,
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof editImage>>,
    TError,
    void,
    TContext
> => {
    return useMutation(getEditImageMutationOptions(options), queryClient);
};
/**
 * @summary Delete a specific image
 */
export type deleteImageResponse200 = {
    data: undefined;
    status: 200;
};

export type deleteImageResponse401 = {
    data: ProblemSchema;
    status: 401;
};

export type deleteImageResponse404 = {
    data: ProblemSchema;
    status: 404;
};

export type deleteImageResponse500 = {
    data: ProblemSchema;
    status: 500;
};

export type deleteImageResponseSuccess = deleteImageResponse200 & {
    headers: Headers;
};
export type deleteImageResponseError = (
    | deleteImageResponse401
    | deleteImageResponse404
    | deleteImageResponse500
) & {
    headers: Headers;
};

export type deleteImageResponse =
    | deleteImageResponseSuccess
    | deleteImageResponseError;

export const getDeleteImageUrl = (id: string) => {
    return `/api/v1/images/${id}`;
};

export const deleteImage = async (
    id: string,
    options?: RequestInit,
): Promise<deleteImageResponse> => {
    return customFetch<deleteImageResponse>(getDeleteImageUrl(id), {
        ...options,
        method: "DELETE",
    });
};

export const getDeleteImageMutationOptions = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(options?: {
    mutation?: CreateMutationOptions<
        Awaited<ReturnType<typeof deleteImage>>,
        TError,
        { id: string },
        TContext
    >;
    request?: SecondParameter<typeof customFetch>;
}): CreateMutationOptions<
    Awaited<ReturnType<typeof deleteImage>>,
    TError,
    { id: string },
    TContext
> => {
    const mutationKey = ["deleteImage"];
    const { mutation: mutationOptions, request: requestOptions } = options
        ? options.mutation &&
          "mutationKey" in options.mutation &&
          options.mutation.mutationKey
            ? options
            : { ...options, mutation: { ...options.mutation, mutationKey } }
        : { mutation: { mutationKey }, request: undefined };

    const mutationFn: MutationFunction<
        Awaited<ReturnType<typeof deleteImage>>,
        { id: string }
    > = (props) => {
        const { id } = props ?? {};

        return deleteImage(id, requestOptions);
    };

    return { mutationFn, ...mutationOptions };
};

export type DeleteImageMutationResult = NonNullable<
    Awaited<ReturnType<typeof deleteImage>>
>;

export type DeleteImageMutationError = ErrorType<ProblemSchema>;

/**
 * @summary Delete a specific image
 */
export const useDeleteImage = <
    TError = ErrorType<ProblemSchema>,
    TContext = unknown,
>(
    options?: {
        mutation?: CreateMutationOptions<
            Awaited<ReturnType<typeof deleteImage>>,
            TError,
            { id: string },
            TContext
        >;
        request?: SecondParameter<typeof customFetch>;
    },
    queryClient?: QueryClient,
): CreateMutationResult<
    Awaited<ReturnType<typeof deleteImage>>,
    TError,
    { id: string },
    TContext
> => {
    return useMutation(getDeleteImageMutationOptions(options), queryClient);
};
