
import * as t from 'io-ts';
import { Context, ValidationError } from 'io-ts';
import { isRight, fold } from 'fp-ts/Either';

const getContextPath = (context: Context): string => context.map(({ key }, index) => {
    if (index === 0 && key === '') {
        return '.';
    }

    return key;
}).join('/');

export interface MessageType {
    path: string,
    value: any,
    expected: string,
}

const failure = (es: Array<ValidationError>): Array<MessageType> => es.map((e: ValidationError): MessageType => {
    const lastTypeName = e.context[e.context.length - 1].type.name;

    return {
        path: getContextPath(e.context),
        value: e.value,
        expected: lastTypeName
    };
});

const success = (): Array<MessageType> => {
    return []
};

export type ValidatorResultType<A> = {
    type: 'ok',
    data: A
} | {
    type: 'error',
    message: Array<MessageType>
};

type Validator<A> = (data: unknown) => ValidatorResultType<A>;

export const buildValidator = <A>(decoder: t.Type<A>): Validator<A> => {
    return (dataIn: unknown): ValidatorResultType<A> => {
        const decodeResult = decoder.decode(dataIn);

        if (isRight(decodeResult)) {
            return {
                type: 'ok',
                data: decodeResult.right
            };
        }

        const errorDecodeInfo = fold(failure, success)(decodeResult);

        return {
            type: 'error',
            message: errorDecodeInfo
        };
    };
};

export const buildValidatorWithUnwrap = <A>(label: string, decoder: t.Type<A>): ((dataIn: unknown) => A) => {
    const validator = buildValidator(decoder);

    return (dataIn: unknown): A => {
        const result = validator(dataIn);

        if (result.type === 'ok') {
            return result.data;
        }

        console.error({
            label: `Decoder '${label}'`,
            dataIn,
            errorDecodeInfo: result.message
        });

        throw new Error(JSON.stringify({
            label: `Decoder '${label}'`,
            errorDecodeInfo: result.message
        }));
    };
};

/*
const rr = t.interface({
    name: t.string,
    age: t.number
});

const decodeAA = buildValidator(rr);

type AA = t.TypeOf<typeof rr>;

const b = decodeAA({});

if (b.type === 'ok') {
    b.data.age;
} else {
    b.message
}
// type AA2 = t.TypeOf<ReturnType<typeof decodeAA>>;
*/