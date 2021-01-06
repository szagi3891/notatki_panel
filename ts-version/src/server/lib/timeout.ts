export const timeout = async (timeout: number): Promise<void> => {
    return new Promise((resolve: (data: void) => void) => {
        setTimeout(resolve, timeout);
    });
};
