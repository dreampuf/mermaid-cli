// Basic polyfills and initialization for the JavaScript runtime
globalThis.console = {
    log: function(...args) {
        Deno.core.ops.op_log(args.map(a => String(a)).join(' '));
    },
    error: function(...args) {
        Deno.core.ops.op_log('[ERROR] ' + args.map(a => String(a)).join(' '));
    },
    warn: function(...args) {
        Deno.core.ops.op_log('[WARN] ' + args.map(a => String(a)).join(' '));
    },
    info: function(...args) {
        Deno.core.ops.op_log('[INFO] ' + args.map(a => String(a)).join(' '));
    },
    debug: function(...args) {
        Deno.core.ops.op_log('[DEBUG] ' + args.map(a => String(a)).join(' '));
    }
};

// Basic timer polyfills
let timerId = 0;
const timers = new Map();

globalThis.setTimeout = function(callback, delay) {
    const id = ++timerId;
    // For now, we'll execute immediately as we don't have real async timers
    // In a real implementation, you'd want to integrate with tokio timers
    if (delay === 0) {
        Promise.resolve().then(callback);
    } else {
        // Simulate delay with a promise
        new Promise(resolve => {
            // This is a simplified version
            resolve();
        }).then(callback);
    }
    return id;
};

globalThis.clearTimeout = function(id) {
    timers.delete(id);
};

globalThis.setInterval = function(callback, delay) {
    const id = ++timerId;
    // Simplified implementation
    return id;
};

globalThis.clearInterval = function(id) {
    timers.delete(id);
};

// Basic fetch polyfill (if needed by Mermaid)
globalThis.fetch = async function(url, options) {
    throw new Error('Fetch is not supported in this environment');
};

// RequestAnimationFrame polyfill
globalThis.requestAnimationFrame = function(callback) {
    return setTimeout(callback, 16);
};

globalThis.cancelAnimationFrame = function(id) {
    clearTimeout(id);
};

// Basic crypto polyfill for UUID generation
globalThis.crypto = {
    randomUUID: function() {
        return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
            const r = Math.random() * 16 | 0;
            const v = c === 'x' ? r : (r & 0x3 | 0x8);
            return v.toString(16);
        });
    },
    getRandomValues: function(array) {
        for (let i = 0; i < array.length; i++) {
            array[i] = Math.floor(Math.random() * 256);
        }
        return array;
    }
};

// Add Promise polyfills if needed
if (typeof Promise.allSettled === 'undefined') {
    Promise.allSettled = function(promises) {
        return Promise.all(
            promises.map(p => 
                Promise.resolve(p).then(
                    value => ({ status: 'fulfilled', value }),
                    reason => ({ status: 'rejected', reason })
                )
            )
        );
    };
}

// Add structuredClone polyfill
globalThis.structuredClone = globalThis.structuredClone || function(obj) {
    return JSON.parse(JSON.stringify(obj));
};

// Add Error.stackTraceLimit
if (typeof Error.stackTraceLimit === 'undefined') {
    Error.stackTraceLimit = 10;
}