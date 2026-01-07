pub(crate) fn rule(
    build_ctx: &crate::builder::BuildCtx,
    node: &rnix::SyntaxNode,
) -> std::collections::LinkedList<crate::builder::Step> {
    let mut steps = std::collections::LinkedList::new();
    let mut children = crate::children::Children::new(build_ctx, node);
    while children.has_next() {
        children.drain_trivia(|_| {});
        if let Some(child) = children.get_next() {
            steps.push_back(crate::builder::Step::Format(child));
        }
    }
    steps
}
