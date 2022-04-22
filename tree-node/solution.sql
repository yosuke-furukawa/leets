SELECT Tree.id, IF(isnull(Tree.p_id), 'Root', IF(Tree.id in (SELECT p_id From Tree), 'Inner', 'Leaf')) Type FROM Tree;
