-- MySQL Script generated by MySQL Workbench
-- Fri 28 Jun 2019 06:46:39 PM CST
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='TRADITIONAL,ALLOW_INVALID_DATES';

-- -----------------------------------------------------
-- Table `mwenglish`.`users`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`users` (
  `id` VARCHAR(32) NOT NULL COMMENT '用户编号',
  `phone` VARCHAR(11) NOT NULL COMMENT '用户登录的手机号',
  `password` VARCHAR(50) NOT NULL COMMENT '用户登录密码',
  `nickname` VARCHAR(50) NULL DEFAULT '' COMMENT '显示的用户昵称',
  `avatar` VARCHAR(255) NULL DEFAULT '' COMMENT '头像地址',
  `is_admin` TINYINT NOT NULL DEFAULT 0 COMMENT '是否为管理员，0为否1为真',
--   `is_active` TINYINT NOT NULL DEFAULT 0,
--   `sms_code` VARCHAR(5) NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `last_modified_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '上次修改时间',
  PRIMARY KEY (`id`),
  UNIQUE INDEX `phone_UNIQUE` (`phone` ASC),
  UNIQUE INDEX `nickname_UNIQUE` (`nickname` ASC))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`exam_manage`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`exam_manage` (
  `id` VARCHAR(32) NOT NULL COMMENT '考试编号',
  `exam_name` VARCHAR(45) NOT NULL COMMENT '考试名称',
  `exam_date` DATETIME NOT NULL COMMENT '考试时间',
  `exam_end_date` DATETIME NOT NULL COMMENT '考试结束日期',
  `exam_last_time` INT NOT NULL COMMENT '考试持续时间',
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`question_type`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`question_type` (
  `no` INT NOT NULL,
  `type` VARCHAR(45) NOT NULL COMMENT '题库类型\n',
  PRIMARY KEY (`no`),
  UNIQUE INDEX `type_UNIQUE` (`type` ASC))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`subject_info`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`subject_info` (
  `id` VARCHAR(32) NOT NULL COMMENT '课程编号',
  `subject` VARCHAR(20) NOT NULL COMMENT '课程名',
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`paper_manage`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`paper_manage` (
  `id` VARCHAR(32) NOT NULL COMMENT '试卷编号',
  `exam_id` VARCHAR(32) NOT NULL COMMENT '考试编号',
  `subject` VARCHAR(20) NOT NULL COMMENT '科目',
  `question_type_no` INT NOT NULL COMMENT '试题类型',
  `question_id` VARCHAR(32) NOT NULL COMMENT '试题编号',
  `question_score` INT NOT NULL COMMENT '试题分数',
  PRIMARY KEY (`id`),
  INDEX `fk_paper_manage_exam_manage1_idx` (`exam_id` ASC),
  INDEX `fk_paper_manage_question_type1_idx` (`question_type_no` ASC),
  INDEX `fk_paper_manage_subject_info1_idx` (`subject` ASC),
  CONSTRAINT `fk_paper_manage_exam_manage1`
    FOREIGN KEY (`exam_id`)
    REFERENCES `mwenglish`.`exam_manage` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_paper_manage_question_type1`
    FOREIGN KEY (`question_type_no`)
    REFERENCES `mwenglish`.`question_type` (`no`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_paper_manage_subject_info1`
    FOREIGN KEY (`subject`)
    REFERENCES `mwenglish`.`subject_info` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`score_manage`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`score_manage` (
  `id` VARCHAR(32) NOT NULL COMMENT '成绩编号',
  `exam_id` VARCHAR(32) NOT NULL COMMENT '考试编号',
  `user_id` VARCHAR(32) NOT NULL COMMENT '用户编号',
  `subject_id` VARCHAR(32) NOT NULL COMMENT '课程编号',
  `score` INT NOT NULL COMMENT '成绩',
  PRIMARY KEY (`id`),
  INDEX `fk_score_manage_users1_idx` (`user_id` ASC),
  INDEX `fk_score_manage_exam_manage1_idx` (`exam_id` ASC),
  INDEX `fk_score_manage_subject_info1_idx` (`subject_id` ASC),
  CONSTRAINT `fk_score_manage_users1`
    FOREIGN KEY (`user_id`)
    REFERENCES `mwenglish`.`users` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_score_manage_exam_manage1`
    FOREIGN KEY (`exam_id`)
    REFERENCES `mwenglish`.`exam_manage` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION,
  CONSTRAINT `fk_score_manage_subject_info1`
    FOREIGN KEY (`subject_id`)
    REFERENCES `mwenglish`.`subject_info` (`id`)
    ON DELETE NO ACTION
    ON UPDATE NO ACTION)
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`single_questions`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`single_questions` (
  `id` VARCHAR(32) NOT NULL COMMENT '问题编号',
  `subject_id` VARCHAR(32) NOT NULL COMMENT '科目编号',
  `question_content` VARCHAR(200) NOT NULL COMMENT '问题内容',
  `answer_a` VARCHAR(100) NOT NULL COMMENT '选项A',
  `answer_b` VARCHAR(100) NOT NULL COMMENT '选项B',
  `answer_c` VARCHAR(100) NULL COMMENT '选项C',
  `answer_d` VARCHAR(100) NULL COMMENT '选项D',
  `corrent_answer` VARCHAR(5) NOT NULL COMMENT '正确选项',
  `value` INT NOT NULL COMMENT '分值',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`tf_questions`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`tf_questions` (
  `id` VARCHAR(32) NOT NULL COMMENT '问题编号\n',
  `subject_id` VARCHAR(32) NOT NULL COMMENT '科目编号',
  `question_content` VARCHAR(200) NOT NULL COMMENT '问题内容',
  `corrent_answer` TINYINT NOT NULL COMMENT '正确答案',
  `value` INT NOT NULL COMMENT '分值',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`fill_questions`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`fill_questions` (
  `id` VARCHAR(32) NOT NULL COMMENT '问题编号',
  `subject_id` VARCHAR(32) NOT NULL COMMENT '科目编号',
  `question_content` VARCHAR(200) NOT NULL COMMENT '问题内容',
  `corrent_answer` VARCHAR(100) NOT NULL COMMENT '正确答案',
  `value` INT NOT NULL COMMENT '分值',
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  PRIMARY KEY (`id`))
ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `mwenglish`.`user_session`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `mwenglish`.`user_session` (
  `token` VARCHAR(32) NOT NULL,
  `pbkey` VARCHAR(200) NOT NULL,
  `pvkey` VARCHAR(200) NOT NULL,
  `user_id` VARCHAR(32) NOT NULL,
  `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`token`))
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
