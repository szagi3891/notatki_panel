
export const checkCommit = (commit: string) => {
    if (commit.length !== 40) {
        throw Error(`Nieprawidłowa długość komita: ${commit.length}`);
    }

    for (const char of commit) {
        if ('a' <= char && char <= 'f') {
            continue;
        }
        if ('0' <= char && char <= '9') {
            continue;
        }

        throw Error(`Nieprawidłowy znak w komicie: ${char}`);
    }
}
