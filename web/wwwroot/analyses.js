const REPORTS = {
    "noun-constituent.determinator": function(report) {
        const sub = encodeURI(report.values.core);
        console.dir(report);
        const article = gendersToArticle(report.values["core.genders"]);
        return {
            "message": `Bij het substantief <i>${sub}</i> hoort het lidwoord <b>${article}</b>.`,
        }
    },
    "noun-constituent.determinator.plurality": function(report) {
        const sub = encodeURI(report.values.core);
        console.dir(report);
        return {
            "message": `De determinator bij <i>${sub}</i> is onjuist, gebruik <b>?</b>.`,
        }
    }
};

function gendersToArticle(genders) {
    console.log(genders);
    if (typeof(genders) === 'string') {
        console.log(genders);
        genders = JSON.parse(genders);
    }

    if (genders["neuter"]) {
        return "het";
    }

    return "de";
}
