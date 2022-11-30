# Overview

This RFC proposes specific guidelines for project item design and communication. 

These guidelines ensure all development teams are aligned, informed, settle the right expectations to each other and work in nice synchrony to deliver company goals.

## Assumptions and expectations

No specific hierarchy items, tags or team organizations are expected.

This RFC assumes there could be out-of-items or out-of-process work. 
So it is considered optional and decided on per team and work at hand.

Expectancy that all cross-team collaboration will adhere to the constraints outlined in this guide. 

In a team, collaboration would adhere to this guide too, until full consensus to do othe flow. Any team member may reject work that is not aligned with the flow outlined here.

RFC does not specifies what happens in some edge cases, example in case of vacations, assuming commons sence is used in these case. 

This RFC assume that we aggreed that `ClickUp`(`CU`) was agreed be `source of truth` of company developmentes processes.  Specifically all work items must entered in CU, with comment outlined before.

Each company member is at least of [Novice Member in CU usage](https://univerasity.clickup.com/page/course-catalog#level_novice,role_member).

## Problem

Currently, development produces wasted output, misses people's expectations on communication channels, request reaction timelines are not clear, and  expected output of one team is not aligned with consumer team. 

We have tooling to make our development more faster and transparenty, but we do not not have agreed on mechanics how to improve our work.

## Solution

We should aligned on flows handling CU items effectiely and efficiently improving on `Problems` outlined above. 

Next is set of guidlines and constrains to operate CU items which are considered good to improvement.

### CU guidlines

`ETA` - is 24 working hours
`Item` - CU item 

#### Commnication

People tagged or mentioned in item or anybody from watching an item are expected to respond within ETA.
GitHub (GH) issues, GH metions or review requests notifications are good help, but not required.
Slack notifications are also good will, but optional.

#### Depedencies

If there is dependency on exteranal our of companu CU work item work or dependcy, it should be accompanied by in CU referencing work item

#### Item existence

Any work which is not considered to be done or planned should not be in CU. 
Repeating, work which is not considered to be done should not be click up item.

CU creation can be dicussed. If dicussion is long and inteream, task to align/decide should be created.

Non of tasks could created or watched by single person. If task was created you should make watch or tag with explanation people who should be aware of its existence and progress.

If task was created as result of communiction in GH or Slack, refere GH item or Slack dicussion (or people from diucssion) in CU item.


#### Item state

If person is not working on CU item during 24 hours (or not going to do so) it changes status to ensure it is NOT  `in progress` item. You should leave comment if you plan to proceed later.
When task is started to work on, it set to equivalent of `progress state`.
If you stop any work on item and not going to do any progress, you unassign task from you. Settle expectations by setting some final end status  or set some intermediate status assigned with assiganation other people or do 3, but remove you from assigned and add comentary why you are doing so.
If task assigned to you and it is not clear why it was happend, you have 24 working hours to ask for clarification.
If any work was considered to be and decided not to be later, it should be set to end state by current assigen or its manager.


### Subitems

If any item takes more then several days to deliver, split it into subtask with resonable delivery goals.
If task grows large because of unvoered new items, limit scope of this task deliver and outline new tasks which scope remaining work. Explain why this is done.
Task created by single person and not tagging any other people should not exists. Consider that to be `at least 1 reviewer` rule for task.  
If work item is assigned to several people and in progress. Specific subitem it outlined assigned to one person.

## Notes

This guidlines compose nicely with per item `definition of done` or/and `acceptance criteria` are outliend in each item.

Having boards wich bring several layers of subtasks into meeting based on `tags` and `labels` well compose with subitems creations guidlines outlined above.

This guideline also compose nicely with automatically recalculaed `Gantt Chart`.

### Ghant Chart



