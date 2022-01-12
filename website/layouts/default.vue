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
</style>
