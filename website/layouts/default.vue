<template>
    <div>
        <LayoutHeader />

        <main>
            <slot />
        </main>
    </div>
</template>

<style>
    body {
        background: gold;
        margin: 0;
    }

    * {
        font-family: "Lucida Sans", "Lucida Sans Regular", "Lucida Grande", "Lucida Sans Unicode", Geneva, Verdana,
            sans-serif;
        /* color: white; */
    }

    main {
        padding: 32px;
    }

    /* TODO: here? */

    .container {
        margin: 64px auto;

        background: white;
        max-width: 480px;
        /* width: fit-content; */

        border-radius: 16px;
        padding: 32px;
        box-shadow: 0 0.5px 4px 0.25px #33333388;
    }

    .input-row {
        margin: 16px 0;
    }

    input {
        padding: 8px;

        border-radius: 8px;
        border: 2px solid black;
    }

    button {
        cursor: pointer;
        transition: all 0.3s;
        width: 100%;

        background: #ffd700;
        color: black;

        border: 2px solid black;
        border-radius: 8px;

        padding: 8px 12px;
        font-size: 20px;
        font-weight: 600;
    }

    button:hover {
        background: #ecc900;
    }
</style>
