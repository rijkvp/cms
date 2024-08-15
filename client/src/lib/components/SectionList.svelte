<script lang="ts">
    import type { Section } from "$lib/models/Section.ts";
    import { sections } from "$lib/store";

    sections.update((s) => {
        s.push({ id: "section1", title: "Section 1" });
        s.push({ id: "section2", title: "Section 2" });
        s.push({ id: "section3", title: "Section 3" });
    });


    function removeSection(id: string) {
        $sections = sections.filter((s, _) => s.id !== id);
    }

    let idInput = "";
    let titleInput = "";

    function addSection() {
        if (sections.find((s) => s.id === idInput)) {
            alert("Section ID already used!");
            return;
        }
        $sections = [...$sections, { id: idInput, title: titleInput }];
        idInput = "";
        titleInput = "";
    }

</script>

<div>
    <h2>Sections</h2>
    {#each $sections as section}
        <div>
            <a href="/sections/{section.id}" >{section.title}</a>
            <button on:click={() => removeSection(section.id)}>Remove</button>
        </div>
    {/each}
    <input type="text" placeholder="Section ID / slug" bind:value={idInput} />
    <input type="text" placeholder="Section title" bind:value={titleInput}/>
    <button
        on:click={() => addSection()}
        >Add Section</button
    >
</div>
