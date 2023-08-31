declare const PostSolution: {
    readonly body: {
        readonly type: "object";
        readonly required: readonly ["capacity", "members", "members_present"];
        readonly properties: {
            readonly capacity: {
                readonly type: "integer";
                readonly format: "uint";
                readonly minimum: 0;
            };
            readonly members: {
                readonly type: "array";
                readonly items: {
                    readonly type: "object";
                    readonly required: readonly ["id", "preferences"];
                    readonly properties: {
                        readonly id: {
                            readonly type: "string";
                        };
                        readonly preferences: {
                            readonly type: "array";
                            readonly items: {
                                readonly type: "string";
                            };
                        };
                    };
                };
            };
            readonly members_present: {
                readonly type: "array";
                readonly items: {
                    readonly type: "string";
                };
            };
        };
        readonly $schema: "http://json-schema.org/draft-04/schema#";
    };
    readonly response: {
        readonly "200": {
            readonly type: "object";
            readonly required: readonly ["members_represented", "members_unrepresented"];
            readonly properties: {
                readonly members_represented: {
                    readonly type: "object";
                    readonly additionalProperties: {
                        readonly type: "string";
                    };
                };
                readonly members_unrepresented: {
                    readonly type: "array";
                    readonly items: {
                        readonly type: "string";
                    };
                    readonly uniqueItems: true;
                };
            };
            readonly $schema: "http://json-schema.org/draft-04/schema#";
        };
    };
};
export { PostSolution };
